#![cfg(test)]

use crate::{Data, F, Unsigned};
use std::sync::atomic::{AtomicBool, Ordering};

const COUNT: usize = if cfg!(miri) { 10 } else { 1000 };

fn verify<T, const N: usize>(core: F<T>, data: &[Vec<T>; N], test: F<T>)
where
    T: Unsigned,
{
    for vec in data {
        for &value in vec {
            let okay = AtomicBool::new(false);
            core(value, &|expected| {
                test(value, &|actual| {
                    assert_eq!(expected, actual);
                    okay.store(true, Ordering::Relaxed);
                });
            });
            assert!(okay.into_inner());
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
    let data = Data::random(COUNT);

    for imp in crate::IMPLS {
        if imp.name == "core" || imp.name == "null" {
            continue;
        }
        if let Some(imp) = imp.u32 {
            verify(core.u32.unwrap(), &data.u32, imp);
        }
        if let Some(imp) = imp.u64 {
            verify(core.u64.unwrap(), &data.u64, imp);
        }
        if let Some(imp) = imp.u128 {
            verify(core.u128.unwrap(), &data.u128, imp);
        }
    }
}
