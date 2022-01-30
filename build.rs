use std::env;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    if let Ok(version) = env::var("DEP_CEC_LIBCEC_VERSION_MAJOR") {
        println!("cargo:rustc-cfg=abi{}", version);
    } else {
        panic!("libcec-sys did not provide CEC version");
    }
}
