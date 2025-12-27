#![cfg_attr(itoa_benchmark_build_probe, feature(int_format_into))]

use core::fmt::NumBuffer;

pub fn u32toa_numbuffer(value: u32, f: &dyn Fn(&str)) {
    f(value.format_into(&mut NumBuffer::new()));
}

pub fn u64toa_numbuffer(value: u64, f: &dyn Fn(&str)) {
    f(value.format_into(&mut NumBuffer::new()));
}

pub fn u128toa_numbuffer(value: u128, f: &dyn Fn(&str)) {
    f(value.format_into(&mut NumBuffer::new()));
}

// Include in sccache cache key.
#[cfg(itoa_benchmark_build_probe)]
const _: Option<&str> = option_env!("RUSTC_BOOTSTRAP");
