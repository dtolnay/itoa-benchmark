#![cfg_attr(int_format_into, feature(int_format_into))]
#![allow(
    clippy::cast_possible_truncation,
    clippy::cast_precision_loss,
    clippy::cast_ptr_alignment,
    clippy::cast_sign_loss,
    clippy::let_and_return,
    clippy::many_single_char_names,
    clippy::nonminimal_bool,
    clippy::similar_names,
    clippy::too_many_lines,
    clippy::unreadable_literal
)]

mod args;
mod bcd16;
mod bcd4;
mod bcd8;
mod branchlut;
mod branchlut2;
mod count;
mod countdecimaldigit;
mod countlut;
mod digitslut;
mod itoa_jeaiii;
mod itoa_ljust;
mod lut;
mod mwilson;
mod naive;
#[cfg(int_format_into)]
mod numbuffer;
mod test_all;
mod tmueller;
mod unnamed;
mod unrolledlut;
mod unsigned;
mod yy;

use crate::args::Type;
use crate::unsigned::Unsigned;
use anyhow::Result;
use arrayvec::ArrayString;
use rand::SeedableRng as _;
use rand::distr::{Distribution as _, Uniform};
use rand::rngs::SmallRng;
use std::any;
use std::fmt::Write as _;
use std::hint;
use std::time::{Duration, Instant};

const COUNT: usize = if cfg!(miri) { 20 } else { 100_000 };
const TRIALS: usize = if cfg!(miri) { 1 } else { 8 };
const PASSES: usize = if cfg!(miri) { 1 } else { 25 };

struct Data {
    u32: [Vec<u32>; 10],
    u64: [Vec<u64>; 20],
    u128: [Vec<u128>; 39],
}

impl Data {
    fn random(count: usize) -> Self {
        let mut rng = SmallRng::seed_from_u64(1);
        let mut data = Data {
            u32: [const { Vec::new() }; 10],
            u64: [const { Vec::new() }; 20],
            u128: [const { Vec::new() }; 39],
        };
        fill(&mut rng, &mut data.u32, count);
        fill(&mut rng, &mut data.u64, count);
        fill(&mut rng, &mut data.u128, count);
        data
    }
}

type F<T> = fn(T, &dyn Fn(&str));

#[derive(Copy, Clone)]
struct Impl {
    name: &'static str,
    u32: Option<F<u32>>,
    u64: Option<F<u64>>,
    u128: Option<F<u128>>,
}

static IMPLS: &[Impl] = &[
    Impl {
        name: "core",
        u32: Some(|value, f| {
            let mut buffer = ArrayString::<10>::new();
            write!(buffer, "{value}").unwrap();
            f(&buffer);
        }),
        u64: Some(|value, f| {
            let mut buffer = ArrayString::<20>::new();
            write!(buffer, "{value}").unwrap();
            f(&buffer);
        }),
        u128: Some(|value, f| {
            let mut buffer = ArrayString::<39>::new();
            write!(buffer, "{value}").unwrap();
            f(&buffer);
        }),
    },
    #[cfg(int_format_into)]
    Impl {
        name: "NumBuffer",
        u32: Some(numbuffer::u32toa_numbuffer),
        u64: Some(numbuffer::u64toa_numbuffer),
        u128: Some(numbuffer::u128toa_numbuffer),
    },
    Impl {
        name: "itoa",
        u32: Some(|value, f| f(itoa::Buffer::new().format(value))),
        u64: Some(|value, f| f(itoa::Buffer::new().format(value))),
        u128: Some(|value, f| f(itoa::Buffer::new().format(value))),
    },
    Impl {
        name: "null",
        u32: Some(|_value, f| f("")),
        u64: Some(|_value, f| f("")),
        u128: Some(|_value, f| f("")),
    },
    Impl {
        name: "branchlut",
        u32: None,
        u64: Some(branchlut::u64toa_branchlut),
        u128: None,
    },
    Impl {
        name: "branchlut2",
        u32: None,
        u64: Some(branchlut2::u64toa_branchlut2),
        u128: None,
    },
    Impl {
        name: "count",
        u32: None,
        u64: Some(count::u64toa_count),
        u128: None,
    },
    Impl {
        name: "countlut",
        u32: None,
        u64: Some(countlut::u64toa_countlut),
        u128: None,
    },
    Impl {
        name: "lut",
        u32: None,
        u64: Some(lut::u64toa_lut),
        u128: None,
    },
    Impl {
        name: "naive",
        u32: None,
        u64: Some(naive::u64toa_naive),
        u128: None,
    },
    Impl {
        name: "amartin",
        u32: None,
        u64: Some(itoa_ljust::u64toa_amartin),
        u128: None,
    },
    Impl {
        name: "bcd4",
        u32: Some(bcd4::u32toa_bcd4),
        u64: Some(bcd4::u64toa_bcd4),
        u128: None,
    },
    Impl {
        name: "bcd8",
        u32: None,
        u64: Some(bcd8::u64toa_bcd8),
        u128: None,
    },
    Impl {
        name: "bcd16",
        u32: None,
        u64: Some(bcd16::u64toa_bcd16),
        u128: None,
    },
    Impl {
        name: "jeaiii",
        u32: None,
        u64: Some(itoa_jeaiii::u64toa_jeaiii),
        u128: None,
    },
    Impl {
        name: "mwilson",
        u32: None,
        u64: Some(mwilson::u64toa_mwilson),
        u128: None,
    },
    Impl {
        name: "tmueller",
        u32: Some(tmueller::u32toa_tmueller),
        u64: Some(tmueller::u64toa_tmueller),
        u128: None,
    },
    Impl {
        name: "unnamed",
        u32: None,
        u64: Some(unnamed::u64toa_unnamed),
        u128: None,
    },
    Impl {
        name: "unrolledlut",
        u32: None,
        u64: Some(unrolledlut::u64toa_unrolledlut),
        u128: None,
    },
    Impl {
        name: "yy",
        u32: None,
        u64: Some(yy::u64toa_yy),
        u128: None,
    },
];

fn fill<T, const N: usize>(rng: &mut SmallRng, data: &mut [Vec<T>; N], count: usize)
where
    T: Unsigned,
{
    for (i, vec) in data.iter_mut().enumerate() {
        let lo = T::TEN.pow(i as u32) - T::from(i == 0);
        let hi = T::TEN
            .saturating_pow(i as u32 + 1)
            .wrapping_add(T::ONE)
            .saturating_sub(T::ONE)
            .wrapping_sub(T::ONE);
        let distr = Uniform::try_from(lo..=hi).unwrap();
        vec.reserve_exact(count);
        for _ in 0..count {
            vec.push(distr.sample(rng));
        }
    }
}

fn measure<T, const N: usize>(data: &[Vec<T>; N], test: F<T>)
where
    T: Unsigned,
{
    println!("  {}", any::type_name::<T>());
    for (i, vec) in data.iter().enumerate() {
        let mut duration = Duration::MAX;
        for _trial in 0..TRIALS {
            let begin = Instant::now();
            for _pass in 0..PASSES {
                for &value in vec {
                    test(value, &|repr| {
                        hint::black_box(repr);
                    });
                }
            }
            duration = Ord::min(duration, begin.elapsed());
        }
        println!(
            "    ({}, {:.2})",
            i + 1,
            duration.as_secs_f64() * 1e9 / (PASSES * vec.len()) as f64,
        );
    }
}

fn main() -> Result<()> {
    let data = Data::random(COUNT);
    let mut prev_name = None;

    for (name, f) in args::parse()? {
        if prev_name != Some(name) {
            println!("{newline}{name}", newline = prev_name.map_or("", |_| "\n"));
            prev_name = Some(name);
        }
        match f {
            Type::U32(f) => measure(&data.u32, f),
            Type::U64(f) => measure(&data.u64, f),
            Type::U128(f) => measure(&data.u128, f),
        }
    }

    Ok(())
}
