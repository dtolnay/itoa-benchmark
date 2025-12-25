use crate::digitslut::DIGITS_LUT;
use std::mem::MaybeUninit;
use std::slice;

pub fn u64toa_branchlut(mut value: u64, f: &dyn Fn(&str)) {
    let mut buffer = [MaybeUninit::<u8>::uninit(); 20];
    let mut out = buffer.as_mut_ptr().cast::<u8>();

    if value < 100000000 {
        let v = value as u32;
        if v < 10000 {
            let d1 = (v / 100) << 1;
            let d2 = (v % 100) << 1;

            if v >= 1000 {
                unsafe {
                    *out = *DIGITS_LUT.get_unchecked(d1 as usize);
                    out = out.add(1);
                }
            }
            if v >= 100 {
                unsafe {
                    *out = *DIGITS_LUT.get_unchecked(d1 as usize + 1);
                    out = out.add(1);
                }
            }
            if v >= 10 {
                unsafe {
                    *out = *DIGITS_LUT.get_unchecked(d2 as usize);
                    out = out.add(1);
                }
            }
            unsafe {
                *out = *DIGITS_LUT.get_unchecked(d2 as usize + 1);
                out = out.add(1);
            }
        } else {
            // value = bbbbcccc
            let b = v / 10000;
            let c = v % 10000;

            let d1 = (b / 100) << 1;
            let d2 = (b % 100) << 1;

            let d3 = (c / 100) << 1;
            let d4 = (c % 100) << 1;

            if value >= 10000000 {
                unsafe {
                    *out = *DIGITS_LUT.get_unchecked(d1 as usize);
                    out = out.add(1);
                }
            }
            if value >= 1000000 {
                unsafe {
                    *out = *DIGITS_LUT.get_unchecked(d1 as usize + 1);
                    out = out.add(1);
                }
            }
            if value >= 100000 {
                unsafe {
                    *out = *DIGITS_LUT.get_unchecked(d2 as usize);
                    out = out.add(1);
                }
            }
            unsafe {
                *out = *DIGITS_LUT.get_unchecked(d2 as usize + 1);
                out = out.add(1);

                *out = *DIGITS_LUT.get_unchecked(d3 as usize);
                out = out.add(1);
                *out = *DIGITS_LUT.get_unchecked(d3 as usize + 1);
                out = out.add(1);
                *out = *DIGITS_LUT.get_unchecked(d4 as usize);
                out = out.add(1);
                *out = *DIGITS_LUT.get_unchecked(d4 as usize + 1);
                out = out.add(1);
            }
        }
    } else if value < 10000000000000000 {
        let v0 = (value / 100000000) as u32;
        let v1 = (value % 100000000) as u32;

        let b0 = v0 / 10000;
        let c0 = v0 % 10000;

        let d1 = (b0 / 100) << 1;
        let d2 = (b0 % 100) << 1;

        let d3 = (c0 / 100) << 1;
        let d4 = (c0 % 100) << 1;

        let b1 = v1 / 10000;
        let c1 = v1 % 10000;

        let d5 = (b1 / 100) << 1;
        let d6 = (b1 % 100) << 1;

        let d7 = (c1 / 100) << 1;
        let d8 = (c1 % 100) << 1;

        if value >= 1000000000000000 {
            unsafe {
                *out = *DIGITS_LUT.get_unchecked(d1 as usize);
                out = out.add(1);
            }
        }
        if value >= 100000000000000 {
            unsafe {
                *out = *DIGITS_LUT.get_unchecked(d1 as usize + 1);
                out = out.add(1);
            }
        }
        if value >= 10000000000000 {
            unsafe {
                *out = *DIGITS_LUT.get_unchecked(d2 as usize);
                out = out.add(1);
            }
        }
        if value >= 1000000000000 {
            unsafe {
                *out = *DIGITS_LUT.get_unchecked(d2 as usize + 1);
                out = out.add(1);
            }
        }
        if value >= 100000000000 {
            unsafe {
                *out = *DIGITS_LUT.get_unchecked(d3 as usize);
                out = out.add(1);
            }
        }
        if value >= 10000000000 {
            unsafe {
                *out = *DIGITS_LUT.get_unchecked(d3 as usize + 1);
                out = out.add(1);
            }
        }
        if value >= 1000000000 {
            unsafe {
                *out = *DIGITS_LUT.get_unchecked(d4 as usize);
                out = out.add(1);
            }
        }
        if value >= 100000000 {
            unsafe {
                *out = *DIGITS_LUT.get_unchecked(d4 as usize + 1);
                out = out.add(1);
            }
        }

        unsafe {
            *out = *DIGITS_LUT.get_unchecked(d5 as usize);
            out = out.add(1);
            *out = *DIGITS_LUT.get_unchecked(d5 as usize + 1);
            out = out.add(1);
            *out = *DIGITS_LUT.get_unchecked(d6 as usize);
            out = out.add(1);
            *out = *DIGITS_LUT.get_unchecked(d6 as usize + 1);
            out = out.add(1);
            *out = *DIGITS_LUT.get_unchecked(d7 as usize);
            out = out.add(1);
            *out = *DIGITS_LUT.get_unchecked(d7 as usize + 1);
            out = out.add(1);
            *out = *DIGITS_LUT.get_unchecked(d8 as usize);
            out = out.add(1);
            *out = *DIGITS_LUT.get_unchecked(d8 as usize + 1);
            out = out.add(1);
        }
    } else {
        let a = (value / 10000000000000000) as u32; // 1 to 1844
        value %= 10000000000000000;

        if a < 10 {
            unsafe {
                *out = b'0' + a as u8;
                out = out.add(1);
            }
        } else if a < 100 {
            let i = a << 1;
            unsafe {
                *out = *DIGITS_LUT.get_unchecked(i as usize);
                out = out.add(1);
                *out = *DIGITS_LUT.get_unchecked(i as usize + 1);
                out = out.add(1);
            }
        } else if a < 1000 {
            unsafe {
                *out = b'0' + (a / 100) as u8;
                out = out.add(1);
            }

            let i = (a % 100) << 1;
            unsafe {
                *out = *DIGITS_LUT.get_unchecked(i as usize);
                out = out.add(1);
                *out = *DIGITS_LUT.get_unchecked(i as usize + 1);
                out = out.add(1);
            }
        } else {
            let i = (a / 100) << 1;
            let j = (a % 100) << 1;
            unsafe {
                *out = *DIGITS_LUT.get_unchecked(i as usize);
                out = out.add(1);
                *out = *DIGITS_LUT.get_unchecked(i as usize + 1);
                out = out.add(1);
                *out = *DIGITS_LUT.get_unchecked(j as usize);
                out = out.add(1);
                *out = *DIGITS_LUT.get_unchecked(j as usize + 1);
                out = out.add(1);
            }
        }

        let v0 = (value / 100000000) as u32;
        let v1 = (value % 100000000) as u32;

        let b0 = v0 / 10000;
        let c0 = v0 % 10000;

        let d1 = (b0 / 100) << 1;
        let d2 = (b0 % 100) << 1;

        let d3 = (c0 / 100) << 1;
        let d4 = (c0 % 100) << 1;

        let b1 = v1 / 10000;
        let c1 = v1 % 10000;

        let d5 = (b1 / 100) << 1;
        let d6 = (b1 % 100) << 1;

        let d7 = (c1 / 100) << 1;
        let d8 = (c1 % 100) << 1;

        unsafe {
            *out = *DIGITS_LUT.get_unchecked(d1 as usize);
            out = out.add(1);
            *out = *DIGITS_LUT.get_unchecked(d1 as usize + 1);
            out = out.add(1);
            *out = *DIGITS_LUT.get_unchecked(d2 as usize);
            out = out.add(1);
            *out = *DIGITS_LUT.get_unchecked(d2 as usize + 1);
            out = out.add(1);
            *out = *DIGITS_LUT.get_unchecked(d3 as usize);
            out = out.add(1);
            *out = *DIGITS_LUT.get_unchecked(d3 as usize + 1);
            out = out.add(1);
            *out = *DIGITS_LUT.get_unchecked(d4 as usize);
            out = out.add(1);
            *out = *DIGITS_LUT.get_unchecked(d4 as usize + 1);
            out = out.add(1);
            *out = *DIGITS_LUT.get_unchecked(d5 as usize);
            out = out.add(1);
            *out = *DIGITS_LUT.get_unchecked(d5 as usize + 1);
            out = out.add(1);
            *out = *DIGITS_LUT.get_unchecked(d6 as usize);
            out = out.add(1);
            *out = *DIGITS_LUT.get_unchecked(d6 as usize + 1);
            out = out.add(1);
            *out = *DIGITS_LUT.get_unchecked(d7 as usize);
            out = out.add(1);
            *out = *DIGITS_LUT.get_unchecked(d7 as usize + 1);
            out = out.add(1);
            *out = *DIGITS_LUT.get_unchecked(d8 as usize);
            out = out.add(1);
            *out = *DIGITS_LUT.get_unchecked(d8 as usize + 1);
            out = out.add(1);
        }
    }

    f(unsafe {
        str::from_utf8_unchecked(slice::from_raw_parts(
            buffer.as_ptr().cast::<u8>(),
            out.offset_from_unsigned(buffer.as_ptr().cast::<u8>()),
        ))
    });
}
