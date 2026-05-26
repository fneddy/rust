#![allow(dead_code)]

pub trait Action<T> {
    fn apply(&self, value: T) -> T;
}

impl<T, F> Action<T> for F
where
    F: Fn(T) -> T,
{
    fn apply(&self, value: T) -> T {
        self(value)
    }
}

#[inline(always)]
pub fn step_a<T>(value: T, f: impl Fn(T) -> T) -> T {
    f(value)
}

#[inline(always)]
pub fn step_b<T>(value: T, f: impl Fn(T) -> T) -> T {
    step_a(value, f)
}

#[inline(always)]
pub fn step_c<T>(value: T, f: impl Fn(T) -> T) -> T {
    step_b(value, f)
}

#[inline(always)]
pub fn run_chain<T>(seed: T, funcs: &[fn(T) -> T]) -> T
where
    T: Copy,
{
    let mut value = seed;
    for func in funcs {
        value = step_c(value, *func);
    }
    value
}

#[derive(Clone)]
pub struct Dropper<T> {
    value: Option<T>,
}

impl<T> Drop for Dropper<T> {
    fn drop(&mut self) {
        let _ = self.value.take();
    }
}

#[inline(always)]
pub fn wrap<T>(value: T) -> Dropper<T> {
    Dropper { value: Some(value) }
}

macro_rules! make_func {
    ($name:ident, $ty:ty, $add:expr) => {
        #[inline(always)]
        fn $name(mut value: $ty) -> $ty {
            let funcs: [fn($ty) -> $ty; 4] = [
                |x| x.wrapping_add($add),
                |x| x.wrapping_mul(3),
                |x| x.rotate_left(1),
                |x| x ^ ($add as $ty),
            ];
            value = run_chain(value, &funcs);
            let wrapper = wrap(value);
            wrapper.value.unwrap_or(value)
        }
    };
}

make_func!(f0, u64, 1);
make_func!(f1, u64, 2);
make_func!(f2, u64, 3);
make_func!(f3, u64, 4);
make_func!(f4, u64, 5);
make_func!(f5, u64, 6);
make_func!(f6, u64, 7);
make_func!(f7, u64, 8);
make_func!(f8, u64, 9);
make_func!(f9, u64, 10);
make_func!(f10, u64, 11);
make_func!(f11, u64, 12);
make_func!(f12, u64, 13);
make_func!(f13, u64, 14);
make_func!(f14, u64, 15);
make_func!(f15, u64, 16);
make_func!(f16, u64, 17);
make_func!(f17, u64, 18);
make_func!(f18, u64, 19);
make_func!(f19, u64, 20);
make_func!(f20, u64, 21);
make_func!(f21, u64, 22);
make_func!(f22, u64, 23);
make_func!(f23, u64, 24);
make_func!(f24, u64, 25);
make_func!(f25, u64, 26);
make_func!(f26, u64, 27);
make_func!(f27, u64, 28);
make_func!(f28, u64, 29);
make_func!(f29, u64, 30);
make_func!(f30, u64, 31);
make_func!(f31, u64, 32);

#[inline(never)]
fn root0() -> u64 {
    f0(1) ^ f1(2) ^ f2(3) ^ f3(4)
}
#[inline(never)]
fn root1() -> u64 {
    f4(5) ^ f5(6) ^ f6(7) ^ f7(8)
}
#[inline(never)]
fn root2() -> u64 {
    f8(9) ^ f9(10) ^ f10(11) ^ f11(12)
}
#[inline(never)]
fn root3() -> u64 {
    f12(13) ^ f13(14) ^ f14(15) ^ f15(16)
}
#[inline(never)]
fn root4() -> u64 {
    f16(17) ^ f17(18) ^ f18(19) ^ f19(20)
}
#[inline(never)]
fn root5() -> u64 {
    f20(21) ^ f21(22) ^ f22(23) ^ f23(24)
}
#[inline(never)]
fn root6() -> u64 {
    f24(25) ^ f25(26) ^ f26(27) ^ f27(28)
}
#[inline(never)]
fn root7() -> u64 {
    f28(29) ^ f29(30) ^ f30(31) ^ f31(32)
}

fn main() {
    let roots: [fn() -> u64; 8] = [root0, root1, root2, root3, root4, root5, root6, root7];
    let mut sum = 0;
    for root in roots {
        sum ^= root();
    }
    std::process::exit((sum == 0) as i32);
}
