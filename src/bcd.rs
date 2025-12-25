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

fn to_bcd16(abcdefghijklmnop: u64) -> u128 {
    let abcdefghijklmnop = u128::from(abcdefghijklmnop);
    let abcdefgh_ijklmnop = abcdefghijklmnop
        + (0x10000000000000000 - 100000000) * ((abcdefghijklmnop * 0xabcc77118461cefd) >> 90);
    let abcd_efgh_ijkl_mnop = abcdefgh_ijklmnop
        + (0x100000000 - 10000)
            * (((abcdefgh_ijklmnop * 0x68db8bb) >> 40) & 0x7fff0000000000007fff);
    let ab_cd_ef_gh_ij_kl_mn_op = abcd_efgh_ijkl_mnop
        + (0x10000 - 100) * (((abcd_efgh_ijkl_mnop * 0x147b) >> 19) & 0x7f0000007f0000007f0000007f);
    let a_b_c_d_e_f_g_h_i_j_k_l_m_n_o_p = ab_cd_ef_gh_ij_kl_mn_op
        + (0x100 - 10)
            * (((ab_cd_ef_gh_ij_kl_mn_op * 0x67) >> 10) & 0xf000f000f000f000f000f000f000f);
    a_b_c_d_e_f_g_h_i_j_k_l_m_n_o_p
}

pub fn u64toa_bcd(value: u64, f: &dyn Fn(&str)) {
    if value < 100 {
        let offset = usize::from(value < 10);
        f(unsafe {
            str::from_utf8_unchecked(
                &crate::digitslut::DIGITS_LUT
                    [value as usize * 2 + offset..(value as usize + 1) * 2],
            )
        });
    } else if value < 100_000_000 {
        let bcd = to_bcd8(value as u32);
        let leading_zeros = bcd.leading_zeros() as usize / 8;
        let bytes = (bcd | 0x30303030_30303030).to_be_bytes();
        f(unsafe { str::from_utf8_unchecked(&bytes[leading_zeros..]) });
    } else if value < 10_000_000_000_000_000 {
        let bcd = to_bcd16(value);
        let leading_zeros = bcd.leading_zeros() as usize / 8;
        let bytes = (bcd | 0x30303030_30303030_30303030_30303030).to_be_bytes();
        f(unsafe { str::from_utf8_unchecked(&bytes[leading_zeros..]) });
    } else {
        f(itoa::Buffer::new().format(value));
    }
}
