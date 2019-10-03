use std::{env, error::Error, fs, path::PathBuf};

fn main() -> Result<(), Box<dyn Error + 'static>> {
    let out_dir = PathBuf::from(env::var_os("OUT_DIR").ok_or("invalid build script invocation")?);

    // Put the linker script somewhere the linker can find it
    fs::write(out_dir.join("memory.x"), &include_bytes!("../memory.x")[..])?;
    println!("cargo:rustc-link-search={}", out_dir.display());

    // Only re-run the build script when memory.x is changed,
    // instead of when any part of the source code changes.
    println!("cargo:rerun-if-changed=../memory.x");

    Ok(())
}
