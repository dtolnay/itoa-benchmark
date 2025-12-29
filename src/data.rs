use crate::Unsigned;
use rand::SeedableRng as _;
use rand::distr::{Distribution as _, Uniform};
use rand::rngs::SmallRng;
use rand::seq::SliceRandom as _;
use std::array;

pub(crate) struct Data {
    pub u32: DataForType<u32, 10>,
    pub u64: DataForType<u64, 20>,
    pub u128: DataForType<u128, 39>,
}

pub(crate) struct DataForType<T, const N: usize> {
    pub count: usize,
    pub mixed: Vec<T>,
    pub by_length: [Vec<T>; N],
    pub unpredictable: bool,
}

impl Data {
    pub(crate) fn random(count: usize, unpredictable: bool) -> Self {
        let mut rng = SmallRng::seed_from_u64(1);
        Data {
            u32: DataForType::random(&mut rng, count, unpredictable),
            u64: DataForType::random(&mut rng, count, unpredictable),
            u128: DataForType::random(&mut rng, count, unpredictable),
        }
    }
}

impl<T, const N: usize> DataForType<T, N>
where
    T: Unsigned,
{
    fn random(rng: &mut SmallRng, count: usize, unpredictable: bool) -> Self {
        let mut mixed = Vec::new();
        let mut by_length = [const { Vec::new() }; N];
        if unpredictable {
            let mixed_distr: [Uniform<T>; N] =
                array::from_fn(|i| uniform_distribution_for_length(i + 1));
            mixed.reserve_exact(count);
            for i in 0..count {
                mixed.push(mixed_distr[i % N].sample(rng));
            }
            mixed.shuffle(rng);
            for (i, vec) in by_length.iter_mut().enumerate() {
                let distr = uniform_distribution_for_length(i + 1);
                vec.reserve_exact(count * 2);
                vec.extend_from_slice(&mixed);
                for _ in 0..count {
                    vec.push(distr.sample(rng));
                }
                vec.shuffle(rng);
            }
        } else {
            for (i, vec) in by_length.iter_mut().enumerate() {
                let distr = uniform_distribution_for_length(i + 1);
                vec.reserve_exact(count);
                for _ in 0..count {
                    vec.push(distr.sample(rng));
                }
            }
        }
        DataForType {
            count,
            mixed,
            by_length,
            unpredictable,
        }
    }
}

fn uniform_distribution_for_length<T>(len: usize) -> Uniform<T>
where
    T: Unsigned,
{
    assert!(len >= 1);
    let lo = T::TEN.saturating_pow(len as u32 - 1) - T::from(len == 1);
    assert!(lo != T::MAX);
    let hi = T::TEN
        .saturating_pow(len as u32)
        .wrapping_add(T::ONE)
        .saturating_sub(T::ONE)
        .wrapping_sub(T::ONE);
    Uniform::new_inclusive(lo, hi).unwrap()
}
