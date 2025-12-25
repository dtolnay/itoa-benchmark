use std::mem::MaybeUninit;
use std::slice;

fn to_bcd8(abcdefgh: u32) -> u64 {
    // An optimization from Xiang JunBo.
    // Three steps BCD. Base 10000 -> base 100 -> base 10.
    // div and mod are evaluated simultaneously as, e.g.
    //   (abcdefgh / 10000) << 32 + (abcdefgh % 10000)
    //      == abcdefgh + (2^32 - 10000) * (abcdefgh / 10000)))
    // where the division on the RHS is implemented by the usual multiply + shift
    // trick and the fractional bits are masked away.
    let abcdefgh = u64::from(abcdefgh);
    let abcd_efgh = abcdefgh + (0x100000000 - 10000) * ((abcdefgh * 0x68db8bb) >> 40);
    let ab_cd_ef_gh = abcd_efgh + (0x10000 - 100) * (((abcd_efgh * 0x147b) >> 19) & 0x7f0000007f);
    let a_b_c_d_e_f_g_h =
        ab_cd_ef_gh + (0x100 - 10) * (((ab_cd_ef_gh * 0x67) >> 10) & 0xf000f000f000f);
    a_b_c_d_e_f_g_h
}

const ZEROS: u64 = 0x30303030_30303030; // 0x30 == '0'

#[repr(C, align(8))]
struct AlignedBuffer([MaybeUninit<u8>; 16]);

pub fn write_significand(value: u64, f: &dyn Fn(&str)) {
    let mut buffer = AlignedBuffer([MaybeUninit::uninit(); 16]);
    let out = buffer.0.as_mut_ptr().cast::<u64>();

    if value < 100_000_000 {
        let bcd = to_bcd8(value as u32);
        let leading_zeros = (bcd | 1).leading_zeros() as usize / 8;
        unsafe {
            *out = bcd.to_be() | ZEROS;
        }
        f(unsafe {
            str::from_utf8_unchecked(slice::from_raw_parts(
                buffer.0.as_ptr().cast::<u8>().add(leading_zeros),
                8 - leading_zeros,
            ))
        });
    } else if value < 10_000_000_000_000_000 {
        let bbccddee = (value / 100_000_000) as u32;
        let ffgghhii = (value % 100_000_000) as u32;
        let bcd = to_bcd8(bbccddee);
        let mut leading_zeros = bcd.leading_zeros() as usize / 8;
        unsafe {
            *out = bcd.to_be() | ZEROS;
        }
        let bcd = to_bcd8(ffgghhii);
        if leading_zeros == 8 {
            leading_zeros += bcd.leading_zeros() as usize / 8;
        }
        unsafe {
            *out.add(1) = bcd.to_be() | ZEROS;
        }
        f(unsafe {
            str::from_utf8_unchecked(slice::from_raw_parts(
                buffer.0.as_ptr().cast::<u8>().add(leading_zeros),
                16 - leading_zeros,
            ))
        });
    } else {
        f(itoa::Buffer::new().format(value));
    }
}
