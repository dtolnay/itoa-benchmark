use anyhow::{Result, bail};
use std::env;

pub struct Args {
    pub benchmark: Vec<(&'static str, Type)>,
    pub unpredictable: bool,
}

pub enum Type {
    U32(crate::F<u32>),
    U64(crate::F<u64>),
    U128(crate::F<u128>),
}

pub fn parse() -> Result<Args> {
    let mut args = env::args_os();
    args.next().unwrap();

    let mut benchmark = Vec::new();
    let mut unpredictable = false;
    'args: for arg in args {
        if let Some(arg) = arg.to_str() {
            if arg == "--unpredictable" {
                unpredictable = true;
                continue;
            }
            let (lib, ty) = match arg.split_once(':') {
                Some((lib, ty)) => (lib, Some(ty)),
                None => (arg, None),
            };
            for imp in crate::IMPLS {
                if imp.name == lib {
                    match ty {
                        None => {
                            if let Some(f) = imp.u32 {
                                benchmark.push((imp.name, Type::U32(f)));
                            }
                            if let Some(f) = imp.u64 {
                                benchmark.push((imp.name, Type::U64(f)));
                            }
                            if let Some(f) = imp.u128 {
                                benchmark.push((imp.name, Type::U128(f)));
                            }
                            continue 'args;
                        }
                        Some("u32") => {
                            if let Some(f) = imp.u32 {
                                benchmark.push((imp.name, Type::U32(f)));
                                continue 'args;
                            }
                        }
                        Some("u64") => {
                            if let Some(f) = imp.u64 {
                                benchmark.push((imp.name, Type::U64(f)));
                                continue 'args;
                            }
                        }
                        Some("u128") => {
                            if let Some(f) = imp.u128 {
                                benchmark.push((imp.name, Type::U128(f)));
                                continue 'args;
                            }
                        }
                        Some(_) => {}
                    }
                }
            }
        }
        bail!("unsupported: {}", arg.display());
    }

    if benchmark.is_empty() {
        for imp in crate::IMPLS {
            if let Some(f) = imp.u32 {
                benchmark.push((imp.name, Type::U32(f)));
            }
            if let Some(f) = imp.u64 {
                benchmark.push((imp.name, Type::U64(f)));
            }
            if let Some(f) = imp.u128 {
                benchmark.push((imp.name, Type::U128(f)));
            }
        }
    }

    Ok(Args {
        benchmark,
        unpredictable,
    })
}
