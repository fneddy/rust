extern crate mono_helper;

use mono_helper::{
    apply_mix, generated_root_0, generated_root_1, generated_root_2, generated_root_3,
    generated_root_4, generated_root_5, generated_root_6, generated_root_7, generated_root_8,
    generated_root_9, generated_root_10, generated_root_11, generated_root_12, generated_root_13,
    generated_root_14, generated_root_15, generated_root_16, generated_root_17, generated_root_18,
    generated_root_19, generated_root_20, generated_root_21, generated_root_22, generated_root_23,
    generated_root_24, generated_root_25, generated_root_26, generated_root_27, generated_root_28,
    generated_root_29, generated_root_30, generated_root_31,
};

#[inline(always)]
fn local_step(value: u64, salt: u64) -> u64 {
    apply_mix(value, salt, 97)
}

#[inline(never)]
fn root0() -> u64 {
    local_step(
        generated_root_0(1) ^ generated_root_1(2) ^ generated_root_2(3) ^ generated_root_3(4),
        1,
    )
}
#[inline(never)]
fn root1() -> u64 {
    local_step(
        generated_root_4(5) ^ generated_root_5(6) ^ generated_root_6(7) ^ generated_root_7(8),
        2,
    )
}
#[inline(never)]
fn root2() -> u64 {
    local_step(
        generated_root_8(9) ^ generated_root_9(10) ^ generated_root_10(11) ^ generated_root_11(12),
        3,
    )
}
#[inline(never)]
fn root3() -> u64 {
    local_step(
        generated_root_12(13)
            ^ generated_root_13(14)
            ^ generated_root_14(15)
            ^ generated_root_15(16),
        4,
    )
}
#[inline(never)]
fn root4() -> u64 {
    local_step(
        generated_root_16(17)
            ^ generated_root_17(18)
            ^ generated_root_18(19)
            ^ generated_root_19(20),
        5,
    )
}
#[inline(never)]
fn root5() -> u64 {
    local_step(
        generated_root_20(21)
            ^ generated_root_21(22)
            ^ generated_root_22(23)
            ^ generated_root_23(24),
        6,
    )
}
#[inline(never)]
fn root6() -> u64 {
    local_step(
        generated_root_24(25)
            ^ generated_root_25(26)
            ^ generated_root_26(27)
            ^ generated_root_27(28),
        7,
    )
}
#[inline(never)]
fn root7() -> u64 {
    local_step(
        generated_root_28(29)
            ^ generated_root_29(30)
            ^ generated_root_30(31)
            ^ generated_root_31(32),
        8,
    )
}

fn main() {
    let roots: [fn() -> u64; 8] = [root0, root1, root2, root3, root4, root5, root6, root7];
    let mut sum = 0;
    for root in roots {
        sum ^= root();
    }
    std::process::exit((sum == 0) as i32);
}
