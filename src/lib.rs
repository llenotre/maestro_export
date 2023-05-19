//! This library is used by build scripts of kernel modules to build them.

use std::env;
use std::path::PathBuf;
use std::process::exit;

/// Prepares the environment for building a kernel module.
pub fn build() {
    let kern_src: PathBuf = match env::var("KERN_SRC") {
        Ok(val) => val.into(),
        Err(env::VarError::NotUnicode(val)) => val.into(),

        Err(env::VarError::NotPresent) => {
            println!("cargo:warning='Missing environment variable KERN_SRC'");
            exit(1);
        }
    };

    let arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap();
    let profile = env::var("PROFILE").unwrap();

	let out_dir = kern_src.join(PathBuf::from(format!("target/{arch}/{profile}")));

    // Host libraries (`macros` for example)
    println!("cargo:rustc-link-search={}/target/{profile}/deps", kern_src.display());

    // The kernel
    println!(
        "cargo:rustc-link-search={}",
        out_dir.display()
    );

    // The kernel's dependencies (`libcore`, etc...)
    println!(
        "cargo:rustc-link-search={}/deps",
        out_dir.display()
    );

    println!(
        "cargo:rustc-env=RUSTFLAGS=-Cforce-frame-pointers=yes -Cno-redzone=yes -Cprefer-dynamic"
    );
}
