//@ ignore-cross-compile
//@ needs-target-std

use std::path::{Path, PathBuf};
use std::{fs, thread};

use run_make_support::rustc;

fn main() {
    let root = Path::new("stress");
    let _ = fs::remove_dir_all(root);
    fs::create_dir_all(root).unwrap();

    let iterations = std::env::var("RUSTC_MONO_STRESS_ITERS")
        .ok()
        .and_then(|value| value.parse::<usize>().ok())
        .unwrap_or(100);
    let workers = std::env::var("RUSTC_MONO_STRESS_WORKERS")
        .ok()
        .and_then(|value| value.parse::<usize>().ok())
        .unwrap_or(4);

    for iteration in 0..iterations {
        let iteration_dir = root.join(format!("iter{iteration}"));
        fs::create_dir_all(&iteration_dir).unwrap();

        let mut handles = Vec::new();
        for worker in 0..workers {
            let worker_dir = iteration_dir.join(format!("worker{worker}"));
            fs::create_dir_all(&worker_dir).unwrap();
            handles.push(thread::spawn(move || run_worker(&worker_dir, iteration, worker)));
        }

        for handle in handles {
            let stderr = handle.join().unwrap();
            if is_suspected_ice(&stderr) {
                panic!("stress compile hit suspected ICE\n{stderr}");
            }
        }
    }
}

fn run_worker(worker_dir: &Path, iteration: usize, worker: usize) -> String {
    let shared = worker_dir.join("shared");
    let incr = worker_dir.join("incremental");
    let out = worker_dir.join("out");
    fs::create_dir_all(&shared).unwrap();
    fs::create_dir_all(&incr).unwrap();
    fs::create_dir_all(&out).unwrap();

    let knobs = knobs_for(iteration, worker);
    let proc_macro_dylib = build_proc_macro(worker_dir, &shared, &knobs, iteration, worker);
    let helper_rlib =
        build_helper(worker_dir, &shared, &proc_macro_dylib, &knobs, iteration, worker);

    let mut all_stderr = String::new();
    for round in 0..3 {
        let stderr = build_bin(
            worker_dir,
            &out,
            &incr,
            &helper_rlib,
            &proc_macro_dylib,
            &knobs,
            iteration,
            worker,
            round,
        );
        all_stderr.push_str(&stderr);
    }

    all_stderr
}

fn build_proc_macro(
    worker_dir: &Path,
    shared: &Path,
    knobs: &Knobs,
    iteration: usize,
    worker: usize,
) -> PathBuf {
    let mut rustc = rustc();
    rustc
        .input("proc_macro.rs")
        .crate_name("mono_macro")
        .crate_type("proc-macro")
        .edition("2021")
        .metadata(&format!("pm-{iteration}-{worker}"))
        .extra_filename(&format!("-pm-{iteration}-{worker}"))
        .out_dir(shared)
        .set_backtrace_level("1")
        .arg("-Zthreads=8")
        .arg(format!("-Ldependency={}", shared.display()));
    apply_knobs(&mut rustc, knobs, false);
    let process = rustc.run();
    let stderr = process.stderr_utf8();
    print!("{stderr}");
    find_proc_macro(shared, "mono_macro")
}

fn build_helper(
    worker_dir: &Path,
    shared: &Path,
    proc_macro_dylib: &Path,
    knobs: &Knobs,
    iteration: usize,
    worker: usize,
) -> PathBuf {
    let mut rustc = rustc();
    rustc
        .input("helper.rs")
        .crate_name("mono_helper")
        .crate_type("rlib")
        .edition("2021")
        .metadata(&format!("helper-{iteration}-{worker}"))
        .extra_filename(&format!("-helper-{iteration}-{worker}"))
        .out_dir(shared)
        .extern_("mono_macro", proc_macro_dylib)
        .set_backtrace_level("1")
        .arg("-Zthreads=8")
        .arg(format!("-Ldependency={}", shared.display()))
        .arg(format!("--cfg=worker_{worker}"));
    apply_knobs(&mut rustc, knobs, true);
    let process = rustc.run();
    let stderr = process.stderr_utf8();
    print!("{stderr}");
    find_rlib(shared, "mono_helper")
}

fn build_bin(
    worker_dir: &Path,
    out: &Path,
    incr: &Path,
    helper_rlib: &Path,
    proc_macro_dylib: &Path,
    knobs: &Knobs,
    iteration: usize,
    worker: usize,
    round: usize,
) -> String {
    let mut rustc = rustc();
    rustc
        .input("main.rs")
        .crate_name(&format!("mono_bin_{worker}"))
        .crate_type("bin")
        .edition("2021")
        .metadata(&format!("bin-{iteration}-{worker}-{round}"))
        .extra_filename(&format!("-bin-{iteration}-{worker}-{round}"))
        .out_dir(out)
        .incremental(incr)
        .extern_("mono_helper", helper_rlib)
        .extern_("mono_macro", proc_macro_dylib)
        .set_backtrace_level("1")
        .arg("-Zthreads=8")
        .arg(format!("-Ldependency={}", helper_rlib.parent().unwrap().display()))
        .arg(format!("--cfg=round_{round}"))
        .arg(format!("--cfg=worker_{worker}"));
    apply_knobs(&mut rustc, knobs, true);
    let process = rustc.run();
    let stderr = process.stderr_utf8();
    print!("{stderr}");
    stderr
}

fn apply_knobs(rustc: &mut run_make_support::Rustc, knobs: &Knobs, allow_lto: bool) {
    rustc
        .arg("-Zinline-mir=yes")
        .arg(format!("-Ccodegen-units={}", knobs.codegen_units))
        .arg(format!("-Copt-level={}", knobs.opt_level))
        .arg(format!("-Cpanic={}", knobs.panic_strategy))
        .arg(format!("-Zshare-generics={}", knobs.share_generics));

    if knobs.embed_bitcode {
        rustc.arg("-Cembed-bitcode=yes");
    }

    if allow_lto {
        if let Some(lto) = knobs.lto {
            rustc.arg(format!("-Clto={lto}"));
        }
    }
}

fn is_suspected_ice(stderr: &str) -> bool {
    stderr.contains("rustc_monomorphize/src/collector.rs")
        || stderr.contains("called `Option::unwrap()` on a `None` value")
        || stderr.contains("the compiler unexpectedly panicked")
}

fn knobs_for(iteration: usize, worker: usize) -> Knobs {
    const CODEGEN_UNITS: [usize; 4] = [4, 8, 16, 32];
    const OPT_LEVELS: [&str; 3] = ["2", "3", "s"];
    const SHARE_GENERICS: [&str; 2] = ["yes", "no"];
    const PANIC_STRATEGIES: [&str; 2] = ["abort", "unwind"];
    const LTOS: [Option<&str>; 3] = [None, Some("thin"), Some("fat")];

    let seed = iteration.wrapping_mul(17).wrapping_add(worker.wrapping_mul(31));
    Knobs {
        codegen_units: CODEGEN_UNITS[seed % CODEGEN_UNITS.len()],
        opt_level: OPT_LEVELS[(seed / 2) % OPT_LEVELS.len()],
        share_generics: SHARE_GENERICS[(seed / 3) % SHARE_GENERICS.len()],
        panic_strategy: PANIC_STRATEGIES[(seed / 5) % PANIC_STRATEGIES.len()],
        lto: LTOS[(seed / 7) % LTOS.len()],
        embed_bitcode: seed % 2 == 0,
    }
}

fn find_proc_macro(dir: &Path, crate_name: &str) -> PathBuf {
    find_by_prefix_and_ext(dir, &format!("lib{crate_name}"), dylib_ext())
}

fn find_rlib(dir: &Path, crate_name: &str) -> PathBuf {
    find_by_prefix_and_ext(dir, &format!("lib{crate_name}"), "rlib")
}

fn find_by_prefix_and_ext(dir: &Path, prefix: &str, ext: &str) -> PathBuf {
    for entry in fs::read_dir(dir).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        let Some(file_name) = path.file_name().and_then(|name| name.to_str()) else {
            continue;
        };
        if file_name.starts_with(prefix) && file_name.ends_with(ext) {
            return path;
        }
    }
    panic!("could not find artifact with prefix `{prefix}` and extension `{ext}` in {dir:?}");
}

fn dylib_ext() -> &'static str {
    if cfg!(target_os = "macos") {
        "dylib"
    } else if cfg!(target_os = "windows") {
        "dll"
    } else {
        "so"
    }
}

struct Knobs {
    codegen_units: usize,
    opt_level: &'static str,
    share_generics: &'static str,
    panic_strategy: &'static str,
    lto: Option<&'static str>,
    embed_bitcode: bool,
}
