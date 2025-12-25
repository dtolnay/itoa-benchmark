use std::mem::MaybeUninit;
use std::slice;

fn u32toa_unnamed(value: u32, f: &dyn Fn(&str)) {
    let mut buffer = [MaybeUninit::<u8>::uninit(); 10];
    let mut out = buffer.as_mut_ptr().cast::<u8>();

    unsafe {
        if 1000000000 <= value {
            *out = ((value / 1000000000) % 10) as u8 + b'0';
            out = out.add(1);
        }
        if 100000000 <= value {
            *out = ((value / 100000000) % 10) as u8 + b'0';
            out = out.add(1);
        }
        if 10000000 <= value {
            *out = ((value / 10000000) % 10) as u8 + b'0';
            out = out.add(1);
        }
        if 1000000 <= value {
            *out = ((value / 1000000) % 10) as u8 + b'0';
            out = out.add(1);
        }
        if 100000 <= value {
            *out = ((value / 100000) % 10) as u8 + b'0';
            out = out.add(1);
        }
        if 10000 <= value {
            *out = ((value / 10000) % 10) as u8 + b'0';
            out = out.add(1);
        }
        if 1000 <= value {
            *out = ((value / 1000) % 10) as u8 + b'0';
            out = out.add(1);
        }
        if 100 <= value {
            *out = ((value / 100) % 10) as u8 + b'0';
            out = out.add(1);
        }
        if 10 <= value {
            *out = ((value / 10) % 10) as u8 + b'0';
            out = out.add(1);
        }

        *out = (value % 10) as u8 + b'0';
        out = out.add(1);

        f(str::from_utf8_unchecked(slice::from_raw_parts(
            buffer.as_ptr().cast::<u8>(),
            out.offset_from_unsigned(buffer.as_ptr().cast::<u8>()),
        )));
    }
}

pub fn u64toa_unnamed(value: u64, f: &dyn Fn(&str)) {
    if (value >> 32) == 0 {
        u32toa_unnamed(value as u32, f);
        return;
    }

    let mut buffer = [MaybeUninit::<u8>::uninit(); 20];
    let mut out = buffer.as_mut_ptr().cast::<u8>();

    unsafe {
        if 10000000000000000000 <= value {
            *out = ((value / 10000000000000000000) % 10) as u8 + b'0';
            out = out.add(1);
        }
        if 1000000000000000000 <= value {
            *out = ((value / 1000000000000000000) % 10) as u8 + b'0';
            out = out.add(1);
        }
        if 100000000000000000 <= value {
            *out = ((value / 100000000000000000) % 10) as u8 + b'0';
            out = out.add(1);
        }
        if 10000000000000000 <= value {
            *out = ((value / 10000000000000000) % 10) as u8 + b'0';
            out = out.add(1);
        }
        if 1000000000000000 <= value {
            *out = ((value / 1000000000000000) % 10) as u8 + b'0';
            out = out.add(1);
        }
        if 100000000000000 <= value {
            *out = ((value / 100000000000000) % 10) as u8 + b'0';
            out = out.add(1);
        }
        if 10000000000000 <= value {
            *out = ((value / 10000000000000) % 10) as u8 + b'0';
            out = out.add(1);
        }
        if 1000000000000 <= value {
            *out = ((value / 1000000000000) % 10) as u8 + b'0';
            out = out.add(1);
        }
        if 100000000000 <= value {
            *out = ((value / 100000000000) % 10) as u8 + b'0';
            out = out.add(1);
        }
        if 10000000000 <= value {
            *out = ((value / 10000000000) % 10) as u8 + b'0';
            out = out.add(1);
        }
        if 1000000000 <= value {
            *out = ((value / 1000000000) % 10) as u8 + b'0';
            out = out.add(1);
        }
        if 100000000 <= value {
            *out = ((value / 100000000) % 10) as u8 + b'0';
            out = out.add(1);
        }
        if 10000000 <= value {
            *out = ((value / 10000000) % 10) as u8 + b'0';
            out = out.add(1);
        }
        if 1000000 <= value {
            *out = ((value / 1000000) % 10) as u8 + b'0';
            out = out.add(1);
        }
        if 100000 <= value {
            *out = ((value / 100000) % 10) as u8 + b'0';
            out = out.add(1);
        }
        if 10000 <= value {
            *out = ((value / 10000) % 10) as u8 + b'0';
            out = out.add(1);
        }
        if 1000 <= value {
            *out = ((value / 1000) % 10) as u8 + b'0';
            out = out.add(1);
        }
        if 100 <= value {
            *out = ((value / 100) % 10) as u8 + b'0';
            out = out.add(1);
        }
        if 10 <= value {
            *out = ((value / 10) % 10) as u8 + b'0';
            out = out.add(1);
        }

        *out = (value % 10) as u8 + b'0';
        out = out.add(1);

        f(str::from_utf8_unchecked(slice::from_raw_parts(
            buffer.as_ptr().cast::<u8>(),
            out.offset_from_unsigned(buffer.as_ptr().cast::<u8>()),
        )));
    }
}
