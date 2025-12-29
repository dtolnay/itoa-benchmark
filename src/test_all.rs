#![cfg(test)]

use crate::{Data, DataForType, F, Impl, Unsigned};
use std::sync::atomic::{AtomicBool, Ordering};

const COUNT: usize = if cfg!(miri) { 10 } else { 1000 };

fn verify<T, const N: usize>(core: &Impl, data: &DataForType<T, N>, get: fn(&Impl) -> Option<F<T>>)
where
    T: Unsigned,
{
    let core = get(core).unwrap();
    for vec in &data.by_length {
        for &value in vec {
            core(value, &|expected| {
                for imp in crate::IMPLS {
                    if imp.name == "core" || imp.name == "null" {
                        continue;
                    }
                    if let Some(test) = get(imp) {
                        let okay = AtomicBool::new(false);
                        test(value, &|actual| {
                            assert_eq!(expected, actual);
                            okay.store(true, Ordering::Relaxed);
                        });
                        assert!(okay.into_inner());
                    }
                }
            });
        }
    }
}

#[test]
fn test_all() {
    let mut core = None;
    for imp in crate::IMPLS {
        if imp.name == "core" {
            core = Some(imp);
            break;
        }
    }

    let core = core.unwrap();
    let data = Data::random(COUNT, false);
    verify(core, &data.u32, |imp| imp.u32);
    verify(core, &data.u64, |imp| imp.u64);
    verify(core, &data.u128, |imp| imp.u128);
}
