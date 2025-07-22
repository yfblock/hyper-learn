use std::io::Result;

fn main() -> Result<()> {
    println!("cargo:rustc-link-arg=-Tlinker.lds");
    println!("cargo:rerun-if-changed=linker.lds");
    println!("cargo:rerun-if-changed=build.rs");

    Ok(())
}
