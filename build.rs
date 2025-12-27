fn main() {
    println!("cargo::rerun-if-changed=build.rs");
    println!("cargo::rustc-check-cfg=cfg(nightly)");
    if rustversion::cfg!(nightly) {
        println!("cargo::rustc-cfg=nightly");
    }
}
