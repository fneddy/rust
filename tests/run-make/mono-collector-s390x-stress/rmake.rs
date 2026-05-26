//@ ignore-cross-compile
//@ only-s390x
//@ needs-target-std

use std::fs;
use std::path::{Path, PathBuf};

use run_make_support::rustc;

fn main() {
    let out_dir = Path::new("stress");
    let _ = fs::remove_dir_all(out_dir);
    fs::create_dir_all(out_dir).unwrap();

    let src = Path::new("main.rs");
    let rustflags = [
        "-Zthreads=8",
        "-Zshare-generics=yes",
        "-Zinline-mir=yes",
        "-Ccodegen-units=16",
        "-Copt-level=3",
        "-Cincremental=stress/incremental",
    ];

    let iterations = std::env::var("RUSTC_MONO_STRESS_ITERS")
        .ok()
        .and_then(|value| value.parse::<usize>().ok())
        .unwrap_or(50);

    for iteration in 0..iterations {
        let metadata = format!("iter{iteration}");
        let extra = format!("-iter{iteration}");
        let stderr = compile_once(src, out_dir, &rustflags, &metadata, &extra);
        if stderr.contains("rustc_monomorphize/src/collector.rs")
            || stderr.contains("internal compiler error")
            || stderr.contains("called `Option::unwrap()` on a `None` value")
        {
            panic!("stress compile hit suspected ICE on iteration {iteration}\n{stderr}");
        }
    }
}

fn compile_once(
    src: &Path,
    out_dir: &Path,
    rustflags: &[&str],
    metadata: &str,
    extra_filename: &str,
) -> String {
    let output_dir = out_dir.join(metadata);
    fs::create_dir_all(&output_dir).unwrap();

    let mut rustc = rustc();
    rustc
        .input(src)
        .crate_name("mono_s390x_stress")
        .crate_type("bin")
        .edition("2021")
        .out_dir(&output_dir)
        .metadata(metadata)
        .extra_filename(extra_filename)
        .set_backtrace_level("1");

    for flag in rustflags {
        rustc.arg(flag);
    }

    let process = rustc.run();
    let stderr = process.stderr_utf8();
    print!("{stderr}");
    stderr
}

fn _keep_pathbuf(_: PathBuf) {}
