use crate::digitslut::DIGITS_LUT as DIGITS;
use std::mem::MaybeUninit;
use std::ptr;
use std::slice;

pub fn u32toa_tmueller(x: u32, f: &dyn Fn(&str)) {
    let mut x = u64::from(x);
    let mut buffer = [MaybeUninit::<u8>::uninit(); 20];
    let mut out = buffer.as_mut_ptr().cast::<u8>();
    if x < 100000 {
        if x < 1000 {
            if x < 10 {
                unsafe {
                    *out = b'0' + x as u8;
                }
                out = unsafe { out.add(1) };
            } else {
                let mut inc = 0u32;
                x *= (0xffff / 100) + 1;
                let mut d = (x >> 16) as u32;
                unsafe {
                    *out = b'0' | d as u8;
                }
                inc |= (-d.cast_signed()).cast_unsigned();
                out = unsafe { out.add((inc >> 31) as usize) };
                x = (x & 0xffff) * 10;
                d = (x >> 16) as u32;
                unsafe {
                    *out = b'0' | d as u8;
                }
                inc |= (-d.cast_signed()).cast_unsigned();
                out = unsafe { out.add((inc >> 31) as usize) };
                x = (x & 0xffff) * 10;
                unsafe {
                    *out = b'0' + (x >> 16) as u8;
                }
                out = unsafe { out.add(1) };
            }
        } else {
            let mut inc = 0u32;
            x *= (0xffffffff / 10000) + 1;
            let d = (x >> 32) as u32;
            unsafe {
                *out = b'0' | d as u8;
            }
            inc |= (-d.cast_signed()).cast_unsigned();
            out = unsafe { out.add((inc >> 31) as usize) };
            x = (x & 0xffffffff) * 100;
            unsafe {
                ptr::copy_nonoverlapping(DIGITS.as_ptr().add(((x >> 32) * 2) as usize), out, 2);
            }
            out = unsafe { out.add(2) };
            x = (x & 0xffffffff) * 100;
            unsafe {
                ptr::copy_nonoverlapping(DIGITS.as_ptr().add(((x >> 32) * 2) as usize), out, 2);
            }
            out = unsafe { out.add(2) };
        }
    } else if x < 10000000 {
        let mut inc = 0u32;
        x *= (0xfffffffffff / 1000000) + 1;
        let d = (x >> 44) as u32;
        unsafe {
            *out = b'0' | d as u8;
        }
        inc |= (-d.cast_signed()).cast_unsigned();
        out = unsafe { out.add((inc >> 31) as usize) };
        x = (x & 0xfffffffffff) * 100;
        unsafe {
            ptr::copy_nonoverlapping(DIGITS.as_ptr().add(((x >> 44) * 2) as usize), out, 2);
        }
        out = unsafe { out.add(2) };
        x = (x & 0xfffffffffff) * 100;
        unsafe {
            ptr::copy_nonoverlapping(DIGITS.as_ptr().add(((x >> 44) * 2) as usize), out, 2);
        }
        out = unsafe { out.add(2) };
        x = (x & 0xfffffffffff) * 100;
        unsafe {
            ptr::copy_nonoverlapping(DIGITS.as_ptr().add(((x >> 44) * 2) as usize), out, 2);
        }
        out = unsafe { out.add(2) };
    } else {
        let mut inc = 0u32;
        x = ((x * 2305843009) >> 29) + 4;
        let mut d = (x >> 32) as u32;
        unsafe {
            *out = b'0' | d as u8;
        }
        inc |= (-d.cast_signed()).cast_unsigned();
        out = unsafe { out.add((inc >> 31) as usize) };
        x = (x & 0xffffffff) * 10;
        d = (x >> 32) as u32;
        unsafe {
            *out = b'0' | d as u8;
        }
        inc |= (-d.cast_signed()).cast_unsigned();
        out = unsafe { out.add((inc >> 31) as usize) };
        x = (x & 0xffffffff) * 100;
        unsafe {
            ptr::copy_nonoverlapping(DIGITS.as_ptr().add(((x >> 32) * 2) as usize), out, 2);
        }
        out = unsafe { out.add(2) };
        x = (x & 0xffffffff) * 100;
        unsafe {
            ptr::copy_nonoverlapping(DIGITS.as_ptr().add(((x >> 32) * 2) as usize), out, 2);
        }
        out = unsafe { out.add(2) };
        x = (x & 0xffffffff) * 100;
        unsafe {
            ptr::copy_nonoverlapping(DIGITS.as_ptr().add(((x >> 32) * 2) as usize), out, 2);
        }
        out = unsafe { out.add(2) };
        x = (x & 0xffffffff) * 100;
        unsafe {
            ptr::copy_nonoverlapping(DIGITS.as_ptr().add(((x >> 32) * 2) as usize), out, 2);
        }
        out = unsafe { out.add(2) };
    }
    f(unsafe {
        str::from_utf8_unchecked(slice::from_raw_parts(
            buffer.as_ptr().cast::<u8>(),
            out.cast_const()
                .offset_from_unsigned(buffer.as_ptr().cast::<u8>()),
        ))
    });
}

static POW_10: [u64; 20] = [
    1,
    10,
    100,
    1000,
    10000,
    100000,
    1000000,
    10000000,
    100000000,
    1000000000,
    10000000000,
    100000000000,
    1000000000000,
    10000000000000,
    100000000000000,
    1000000000000000,
    10000000000000000,
    100000000000000000,
    1000000000000000000,
    10000000000000000000,
];

pub fn u64toa_tmueller(mut v: u64, f: &dyn Fn(&str)) {
    let mut buffer = [MaybeUninit::<u8>::uninit(); 20];
    let mut out = buffer.as_mut_ptr().cast::<u8>();

    if v < 10 {
        unsafe {
            *out = b'0' + v as u8;
            f(str::from_utf8_unchecked(slice::from_raw_parts(
                out.cast_const(),
                1,
            )));
        }
        return;
    }
    let zeros = 64 - v.leading_zeros();
    let mut len = (1233 * zeros) >> 12;
    let p10 = unsafe { *POW_10.get_unchecked(len as usize) };
    if v >= p10 {
        len += 1;
    }
    out = unsafe { out.add(len as usize) };
    while v >= 100 {
        let d100 = v / 100;
        let index = v - d100 * 100;
        v = d100;
        unsafe {
            out = out.sub(2);
            ptr::copy_nonoverlapping(DIGITS.as_ptr().add(index as usize * 2), out, 2);
        }
    }
    if v < 10 {
        unsafe {
            out = out.sub(1);
            *out = b'0' + v as u8;
            f(str::from_utf8_unchecked(slice::from_raw_parts(
                out.cast_const(),
                len as usize,
            )));
        }
        return;
    }
    unsafe {
        out = out.sub(2);
        ptr::copy_nonoverlapping(DIGITS.as_ptr().add(v as usize * 2), out, 2);
        f(str::from_utf8_unchecked(slice::from_raw_parts(
            out.cast_const(),
            len as usize,
        )));
    }
}
