fn to_bcd4(abcd: u16) -> u32 {
    let abcd = u32::from(abcd);
    let ab_cd = abcd + (0x10000 - 100) * ((abcd * 0x147b) >> 19);
    let a_b_c_d = ab_cd + (0x100 - 10) * (((ab_cd * 0x67) >> 10) & 0xf000f);
    a_b_c_d
}

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

pub fn u64toa_bcd(value: u64, f: &dyn Fn(&str)) {
    if value < 10_000 {
        let bcd = to_bcd4(value as u16);
        let leading_zeros = (bcd | 1).leading_zeros() as usize / 8;
        let bytes = (bcd | 0x30303030).to_be_bytes();
        f(unsafe { str::from_utf8_unchecked(&bytes[leading_zeros..]) });
    } else if value < 100_000_000 {
        let bcd_hi = to_bcd4((value / 10_000) as u16);
        let leading_zeros = bcd_hi.leading_zeros() as usize / 8;
        let bcd_lo = to_bcd4((value % 10_000) as u16);
        let bytes = [
            (bcd_hi | 0x30303030).to_be_bytes(),
            (bcd_lo | 0x30303030).to_be_bytes(),
        ];
        f(unsafe { str::from_utf8_unchecked(&bytes.as_flattened()[leading_zeros..]) });
    } else if value < 10_000_000_000_000_000 {
        let bcd_hi = to_bcd8((value / 100_000_000) as u32);
        let leading_zeros = bcd_hi.leading_zeros() as usize / 8;
        let bcd_lo = to_bcd8((value % 100_000_000) as u32);
        let bytes = [
            (bcd_hi | 0x30303030_30303030).to_be_bytes(),
            (bcd_lo | 0x30303030_30303030).to_be_bytes(),
        ];
        f(unsafe { str::from_utf8_unchecked(&bytes.as_flattened()[leading_zeros..]) });
    } else {
        f(itoa::Buffer::new().format(value));
    }
}
