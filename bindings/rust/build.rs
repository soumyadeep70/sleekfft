use std::{env, path::PathBuf};

fn main() {
    let manifest = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());

    // Repo-level include/ directory (contains sleekfft/fft.h and the SIMD headers).
    let c_include = manifest.join("../../include");

    cc::Build::new()
        .std("gnu11")
        .opt_level(3)
        .warnings(true)
        .file("c_src/sleekfft_impl.c")
        .include(&c_include)
        .compile("sleekfft_lib");

    println!("cargo:rerun-if-changed=c_src");
    println!("cargo:rerun-if-changed=../../include");
    println!("cargo:rerun-if-env-changed=CC");
}
