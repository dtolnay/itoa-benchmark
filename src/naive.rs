use std::mem::MaybeUninit;
use std::slice;

pub fn u64toa_naive(mut value: u64, f: &dyn Fn(&str)) {
    let mut temp = [MaybeUninit::<u8>::uninit(); 20];
    let mut p = temp.as_mut_ptr();
    while {
        unsafe {
            *p.cast::<u8>() = (value % 10) as u8 + b'0';
            p = p.add(1);
        }
        value /= 10;
        value > 0
    } {}

    let mut buffer = [MaybeUninit::<u8>::uninit(); 20];
    let mut out = buffer.as_mut_ptr();
    while {
        unsafe {
            p = p.sub(1);
            *out = *p;
            out = out.add(1);
        }
        p.cast_const() != temp.as_ptr()
    } {}

    f(unsafe {
        str::from_utf8_unchecked(slice::from_raw_parts(
            buffer.as_ptr().cast::<u8>(),
            out.offset_from_unsigned(buffer.as_ptr()),
        ))
    });
}
