use crate::countdecimaldigit::count_decimal_digit_64;
use std::mem::MaybeUninit;
use std::slice;

pub fn u64toa_count(mut value: u64, f: &dyn Fn(&str)) {
    let digit = count_decimal_digit_64(value);

    let mut buffer = [MaybeUninit::<u8>::uninit(); 20];
    let mut out = unsafe { buffer.as_mut_ptr().add(digit as usize).cast::<u8>() };

    while {
        unsafe {
            out = out.sub(1);
            *out = (value % 10) as u8 + b'0';
        }
        value /= 10;
        value > 0
    } {}

    f(unsafe { str::from_utf8_unchecked(slice::from_raw_parts(out, digit as usize)) });
}
