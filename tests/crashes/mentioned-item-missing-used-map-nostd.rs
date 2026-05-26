// no_std version of mentioned-item-missing-used-map.rs so it can be compiled
// with --target s390x-unknown-linux-gnu without a target sysroot.
// Use --emit=mir or -Zno-codegen to exercise the monomorphization collector
// (where the ICE lives) without needing a linker or stdlib for s390x.
//
// The ICE is in the monomorphization collector / partitioning, which runs
// before codegen, so --emit=mir is sufficient to trigger it.
//
// See mentioned-item-missing-used-map.rs for the full explanation.
//
//@ known-bug: unknown
//@ compile-flags: -Copt-level=1 -Ccodegen-units=4 -Zthreads=8
//@ needs-target: s390x-unknown-linux-gnu

#![crate_type = "lib"]
#![no_std]

#[inline]
pub fn leaf() -> u32 {
    42
}

#[inline]
pub fn middle() -> u32 {
    leaf()
}

pub fn root_a() -> u32 {
    middle()
}

pub fn root_b() -> u32 {
    if false { leaf() } else { 0 }
}
