// unrolledlut.cpp: Fast integer to string conversion by using per-digit-count unrolling and a lookuptable
//
// ===-------- DESCRIPTION --------===
//
// Very fast implementation of uint32_t to string:
// - Automatically takes advantage of two-byte load/store on
//   architectures that support it (memcpy will be optimized).
// - Avoids as many jumps as possible, by unrolling the whole thing for every digit count.
// - Con: Costs some memory for the duplicated instructions of all branches
//
// Further optimization possible:
// - You may reorder the digit-cases, so that the most
//   commonly used cases come first. Currently digit-counts
//   from 7 to 10 are processed first, as they cover ~99.7% of all uint32_t values.
//   By reordering these for your specific needs, you can save one or two extra instructions for these cases.
//
// Copyright (c) 2017 nyronium (nyronium@genthree.io)

use crate::digitslut::DIGITS_LUT as TWO_DIGITS_TO_STR;
use std::mem::MaybeUninit;
use std::slice;

macro_rules! copy_2_digits {
    ($out:ident, $value:ident) => {
        unsafe {
            $out.cast::<u16>().write_unaligned(
                *TWO_DIGITS_TO_STR
                    .as_ptr()
                    .cast::<u16>()
                    .add($value as usize),
            );
            $out = $out.add(2);
        }
    };
}

macro_rules! copy_1_digit {
    ($out:ident, $value:ident) => {
        unsafe {
            *$out = b'0' + $value as u8;
            $out = $out.add(1);
        }
    };
}

macro_rules! unroll_exact_digits_8 {
    ($out:ident, $value:ident) => {
        let mut digits: u32;
        digits = ($value / 1000000) as u32;
        copy_2_digits!($out, digits);
        $value -= digits * 1000000;
        digits = $value / 10000;
        copy_2_digits!($out, digits);
        $value -= digits * 10000;
        digits = $value / 100;
        copy_2_digits!($out, digits);
        $value -= digits * 100;
        copy_2_digits!($out, $value);
    };
}

macro_rules! unroll_remaining_digits_8 {
    ($out:ident, $value:ident, $digits:ident) => {
        $value -= $digits * 100000000;
        $digits = $value / 1000000;
        copy_2_digits!($out, $digits);
        $value -= $digits * 1000000;
        $digits = $value / 10000;
        copy_2_digits!($out, $digits);
        $value -= $digits * 10000;
        $digits = $value / 100;
        copy_2_digits!($out, $digits);
        $value -= $digits * 100;
        copy_2_digits!($out, $value);
        return $out;
    };
}

macro_rules! unroll_remaining_digits_6 {
    ($out:ident, $value:ident, $digits:ident) => {
        $value -= $digits * 1000000;
        $digits = $value / 10000;
        copy_2_digits!($out, $digits);
        $value -= $digits * 10000;
        $digits = $value / 100;
        copy_2_digits!($out, $digits);
        $value -= $digits * 100;
        copy_2_digits!($out, $value);
        return $out;
    };
}

macro_rules! unroll_remaining_digits_4 {
    ($out:ident, $value:ident, $digits:ident) => {
        $value -= $digits * 10000;
        $digits = $value / 100;
        copy_2_digits!($out, $digits);
        $value -= $digits * 100;
        copy_2_digits!($out, $value);
        return $out;
    };
}

macro_rules! unroll_remaining_digits_2 {
    ($out:ident, $value:ident, $digits:ident) => {
        $value -= $digits * 100;
        copy_2_digits!($out, $value);
        return $out;
    };
}

macro_rules! unroll_remaining_digits_0 {
    ($out:ident, $value:ident) => {
        return $out;
    };
}

macro_rules! unroll_digit_pair_9_10 {
    ($out:ident, $value:ident) => {
        let mut digits: u32;
        if ($value >= 1000000000) {
            digits = $value / 100000000;
            copy_2_digits!($out, digits);
            unroll_remaining_digits_8!($out, $value, digits);
        } else {
            digits = $value / 100000000;
            copy_1_digit!($out, digits);
            unroll_remaining_digits_8!($out, $value, digits);
        }
    };
}

macro_rules! unroll_digit_pair_7_8 {
    ($out:ident, $value:ident) => {
        let mut digits: u32;
        if ($value >= 10000000) {
            digits = $value / 1000000;
            copy_2_digits!($out, digits);
            unroll_remaining_digits_6!($out, $value, digits);
        } else {
            digits = $value / 1000000;
            copy_1_digit!($out, digits);
            unroll_remaining_digits_6!($out, $value, digits);
        }
    };
}

macro_rules! unroll_digit_pair_5_6 {
    ($out:ident, $value:ident) => {
        let mut digits: u32;
        if ($value >= 100000) {
            digits = $value / 10000;
            copy_2_digits!($out, digits);
            unroll_remaining_digits_4!($out, $value, digits);
        } else {
            digits = $value / 10000;
            copy_1_digit!($out, digits);
            unroll_remaining_digits_4!($out, $value, digits);
        }
    };
}

macro_rules! unroll_digit_pair_3_4 {
    ($out:ident, $value:ident) => {
        let digits: u32;
        if ($value >= 1000) {
            digits = $value / 100;
            copy_2_digits!($out, digits);
            unroll_remaining_digits_2!($out, $value, digits);
        } else {
            digits = $value / 100;
            copy_1_digit!($out, digits);
            unroll_remaining_digits_2!($out, $value, digits);
        }
    };
}

macro_rules! unroll_digit_pair_1_2 {
    ($out:ident, $value:ident) => {
        if ($value >= 10) {
            copy_2_digits!($out, $value);
            unroll_remaining_digits_0!($out, $value);
        } else {
            copy_1_digit!($out, $value);
            unroll_remaining_digits_0!($out, $value);
        }
    };
}

unsafe fn unrolledlut(mut value: u32, mut out: *mut u8) -> *mut u8 {
    if value >= 100000000 {
        unroll_digit_pair_9_10!(out, value);
    } else if value >= 1000000 {
        unroll_digit_pair_7_8!(out, value);
    } else if value < 100 {
        unroll_digit_pair_1_2!(out, value);
    } else if value < 10000 {
        unroll_digit_pair_3_4!(out, value);
    } else {
        unroll_digit_pair_5_6!(out, value);
    }
}

unsafe fn unrolledlut64(value: u64, mut buffer: *mut u8) -> *mut u8 {
    let mut least_significant = value as u32;
    if u64::from(least_significant) == value {
        return unsafe { unrolledlut(least_significant, buffer) };
    }

    let high12 = value / 100000000;

    /* optimized unrolled recursion */
    least_significant = high12 as u32;
    if u64::from(least_significant) == high12 {
        buffer = unsafe { unrolledlut(least_significant, buffer) };
    } else {
        let high4 = high12 / 100000000;
        buffer = unsafe { unrolledlut(high4 as u32, buffer) };

        let mut digits_15_8 = (high12 - (high4 * 100000000)) as u32;
        unroll_exact_digits_8!(buffer, digits_15_8);
    }

    let mut digits_7_0 = (value - (high12 * 100000000)) as u32;
    unroll_exact_digits_8!(buffer, digits_7_0);
    buffer
}

pub fn u64toa_unrolledlut(value: u64, f: &dyn Fn(&str)) {
    let mut buffer = [MaybeUninit::<u8>::uninit(); 20];
    unsafe {
        let end = unrolledlut64(value, buffer.as_mut_ptr().cast::<u8>());
        f(str::from_utf8_unchecked(slice::from_raw_parts(
            buffer.as_ptr().cast::<u8>(),
            end.cast_const()
                .offset_from_unsigned(buffer.as_ptr().cast::<u8>()),
        )));
    }
}
