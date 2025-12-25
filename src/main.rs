#![allow(
    clippy::cast_possible_truncation,
    clippy::cast_precision_loss,
    clippy::too_many_lines,
    clippy::unreadable_literal
)]

mod branchlut;
mod branchlut2;
mod count;
mod countdecimaldigit;
mod countlut;
mod digitslut;
mod itoa_jeaiii;
mod itoa_ljust_impl;
mod lut;
mod mwilson;
mod naive;
mod test_all;
mod tmueller;
mod unnamed;
mod unrolledlut;
mod unsigned;
mod yy;

use crate::unsigned::Unsigned;
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

fn main() {
    let data = Data::random(COUNT);

    for imp in IMPLS {
        println!("{}", imp.name);
        if let Some(imp) = imp.u32 {
            measure(&data.u32, imp);
        }
        if let Some(imp) = imp.u64 {
            measure(&data.u64, imp);
        }
        if let Some(imp) = imp.u128 {
            measure(&data.u128, imp);
        }
        println!();
    }
}
