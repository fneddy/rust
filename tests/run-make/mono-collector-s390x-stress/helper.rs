use mono_macro::generate_roots;

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

#[inline(always)]
pub fn apply_mix(mut value: u64, salt: u64, seed: u64) -> u64 {
    value = value.wrapping_add(seed).wrapping_add(salt);
    value = value.rotate_left((seed % 31) as u32 + 1);
    value = value.wrapping_mul(3).wrapping_add(salt);
    value ^ seed.wrapping_mul(salt.wrapping_add(1))
}

macro_rules! make_helper_root {
    ($name:ident, $seed:expr) => {
        #[inline(never)]
        pub fn $name(mut value: u64, salt: u64) -> u64 {
            value = apply_mix(value, salt, $seed);
            value = apply_mix(value, salt.wrapping_add(1), $seed + 11);
            value = apply_mix(value, salt.wrapping_add(2), $seed + 29);
            let wrapper = wrap(value);
            wrapper.value.unwrap_or(value)
        }
    };
}

make_helper_root!(helper_root_0, 1);
make_helper_root!(helper_root_1, 2);
make_helper_root!(helper_root_2, 3);
make_helper_root!(helper_root_3, 4);
make_helper_root!(helper_root_4, 5);
make_helper_root!(helper_root_5, 6);
make_helper_root!(helper_root_6, 7);
make_helper_root!(helper_root_7, 8);
make_helper_root!(helper_root_8, 9);
make_helper_root!(helper_root_9, 10);
make_helper_root!(helper_root_10, 11);
make_helper_root!(helper_root_11, 12);
make_helper_root!(helper_root_12, 13);
make_helper_root!(helper_root_13, 14);
make_helper_root!(helper_root_14, 15);
make_helper_root!(helper_root_15, 16);
make_helper_root!(helper_root_16, 17);
make_helper_root!(helper_root_17, 18);
make_helper_root!(helper_root_18, 19);
make_helper_root!(helper_root_19, 20);
make_helper_root!(helper_root_20, 21);
make_helper_root!(helper_root_21, 22);
make_helper_root!(helper_root_22, 23);
make_helper_root!(helper_root_23, 24);
make_helper_root!(helper_root_24, 25);
make_helper_root!(helper_root_25, 26);
make_helper_root!(helper_root_26, 27);
make_helper_root!(helper_root_27, 28);
make_helper_root!(helper_root_28, 29);
make_helper_root!(helper_root_29, 30);
make_helper_root!(helper_root_30, 31);
make_helper_root!(helper_root_31, 32);

generate_roots!(32);
