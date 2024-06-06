fn main() {
    // If we set CARGO_BUILD_VERSION this way, then it will override the default value, which is
    // taken from the `version` in Cargo.toml.
    if let Ok(val) = std::env::var("CARGO_BUILD_VERSION") {
        println!("Using custom version: {}", val);
        println!("cargo:rustc-env=CARGO_PKG_VERSION={}", val);
    }
    println!("cargo:rerun-if-env-changed=CARGO_BUILD_VERSION");
}
