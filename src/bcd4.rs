fn to_bcd4(abcd: u16) -> u32 {
    let abcd = u32::from(abcd);
    let ab_cd = abcd + (0x10000 - 100) * ((abcd * 0x147b) >> 19);
    let a_b_c_d = ab_cd + (0x100 - 10) * (((ab_cd * 0x67) >> 10) & 0xf000f);
    a_b_c_d
}

pub fn u64toa_bcd4(value: u64, f: &dyn Fn(&str)) {
    if value < 10_000 {
        let bcd = to_bcd4(value as u16);
        let leading_zeros = (bcd | 1).leading_zeros() as usize / 8;
        let bytes = (bcd | 0x30303030).to_be_bytes();
        f(unsafe { str::from_utf8_unchecked(&bytes[leading_zeros..]) });
    } else if value < 100_000_000 {
        let bcd_hi = to_bcd4((value / 10_000) as u16);
        let bcd_lo = to_bcd4((value % 10_000) as u16);
        let leading_zeros = bcd_hi.leading_zeros() as usize / 8;
        let bytes = [
            (bcd_hi | 0x30303030).to_be_bytes(),
            (bcd_lo | 0x30303030).to_be_bytes(),
        ];
        f(unsafe { str::from_utf8_unchecked(&bytes.as_flattened()[leading_zeros..]) });
    } else if value < 10_000_000_000_000_000 {
        let hi = (value / 100_000_000) as u32;
        let lo = (value % 100_000_000) as u32;
        let bcd_hi_hi = to_bcd4((hi / 10_000) as u16);
        let bcd_hi_lo = to_bcd4((hi % 10_000) as u16);
        let bcd_lo_hi = to_bcd4((lo / 10_000) as u16);
        let bcd_lo_lo = to_bcd4((lo % 10_000) as u16);
        let leading_zeros =
            ((u64::from(bcd_hi_hi) << 32) | u64::from(bcd_hi_lo)).leading_zeros() as usize / 8;
        let bytes = [
            (bcd_hi_hi | 0x30303030).to_be_bytes(),
            (bcd_hi_lo | 0x30303030).to_be_bytes(),
            (bcd_lo_hi | 0x30303030).to_be_bytes(),
            (bcd_lo_lo | 0x30303030).to_be_bytes(),
        ];
        f(unsafe { str::from_utf8_unchecked(&bytes.as_flattened()[leading_zeros..]) });
    } else {
        f(itoa::Buffer::new().format(value));
    }
}
