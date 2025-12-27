use std::env;
use std::fs;
use std::io::ErrorKind;
use std::iter;
use std::path::Path;
use std::process::{self, Command, Stdio};

fn main() {
    println!("cargo:rerun-if-changed=src/numbuffer.rs");
    println!("cargo:rustc-check-cfg=cfg(int_format_into)");
    println!("cargo:rustc-check-cfg=cfg(itoa_benchmark_build_probe)");

    let int_format_into;
    let consider_rustc_bootstrap;
    if compile_probe(false) {
        // This is a nightly or dev compiler, so it supports unstable
        // features regardless of RUSTC_BOOTSTRAP. No need to rerun build
        // script if RUSTC_BOOTSTRAP is changed.
        int_format_into = true;
        consider_rustc_bootstrap = false;
    } else if let Some(rustc_bootstrap) = env::var_os("RUSTC_BOOTSTRAP") {
        if compile_probe(true) {
            // This is a stable or beta compiler for which the user has set
            // RUSTC_BOOTSTRAP to turn on unstable features. Rerun build
            // script if they change it.
            int_format_into = true;
            consider_rustc_bootstrap = true;
        } else if rustc_bootstrap == "1" {
            // This compiler does not support the integer format_into API in the
            // form that itoa-benchmark expects. No need to pay attention to
            // RUSTC_BOOTSTRAP.
            int_format_into = false;
            consider_rustc_bootstrap = false;
        } else {
            // This is a stable or beta compiler for which RUSTC_BOOTSTRAP
            // is set to restrict the use of unstable features by this
            // crate.
            int_format_into = false;
            consider_rustc_bootstrap = true;
        }
    } else {
        // Without RUSTC_BOOTSTRAP, this compiler does not support the integer
        // format_into API in the form that itoa-benchmark expects, but try
        // again if the user turns on unstable features.
        int_format_into = false;
        consider_rustc_bootstrap = true;
    }

    if int_format_into {
        println!("cargo:rustc-cfg=int_format_into");
    }

    if consider_rustc_bootstrap {
        println!("cargo:rerun-if-env-changed=RUSTC_BOOTSTRAP");
    }
}

fn compile_probe(rustc_bootstrap: bool) -> bool {
    if env::var_os("RUSTC_STAGE").is_some() {
        // We are running inside rustc bootstrap. This is a highly non-standard
        // environment with issues such as:
        //
        //     https://github.com/rust-lang/cargo/issues/11138
        //     https://github.com/rust-lang/rust/issues/114839
        //
        // Let's just not use nightly features here.
        return false;
    }

    let rustc = env::var_os("RUSTC").unwrap();
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let out_subdir = Path::new(&out_dir).join("probe");
    let probefile = Path::new("src").join("numbuffer.rs");

    if let Err(err) = fs::create_dir(&out_subdir)
        && err.kind() != ErrorKind::AlreadyExists
    {
        eprintln!("Failed to create {}: {}", out_subdir.display(), err);
        process::exit(1);
    }

    let rustc_wrapper = env::var_os("RUSTC_WRAPPER").filter(|wrapper| !wrapper.is_empty());
    let rustc_workspace_wrapper =
        env::var_os("RUSTC_WORKSPACE_WRAPPER").filter(|wrapper| !wrapper.is_empty());
    let mut rustc = rustc_wrapper
        .into_iter()
        .chain(rustc_workspace_wrapper)
        .chain(iter::once(rustc));
    let mut cmd = Command::new(rustc.next().unwrap());
    cmd.args(rustc);

    if !rustc_bootstrap {
        cmd.env_remove("RUSTC_BOOTSTRAP");
    }

    cmd.stderr(Stdio::null())
        .arg("--cfg=itoa_benchmark_build_probe")
        .arg("--edition=2024")
        .arg("--crate-name=itoa_benchmark")
        .arg("--crate-type=lib")
        .arg("--cap-lints=allow")
        .arg("--emit=dep-info,metadata")
        .arg("--out-dir")
        .arg(&out_subdir)
        .arg(probefile);

    if let Some(target) = env::var_os("TARGET") {
        cmd.arg("--target").arg(target);
    }

    // If Cargo wants to set RUSTFLAGS, use that.
    if let Ok(rustflags) = env::var("CARGO_ENCODED_RUSTFLAGS")
        && !rustflags.is_empty()
    {
        for arg in rustflags.split('\x1f') {
            cmd.arg(arg);
        }
    }

    match cmd.status() {
        Ok(status) => status.success(),
        Err(_) => false,
    }
}
