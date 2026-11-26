//! (Neptune) The D3D12 Agility SDK exports (`src/agility.rs`): D3D12 finds them on the
//! executable and then loads the runtime from `D3D12\` beside it. Windows 10's in-box
//! runtime stops at shader model 6.5; the debug layer Windows installs is also older than its
//! runtime on some PCs. Only the MSVC linker takes /EXPORT on a binary.
//!
//! Export them when the diagnostics feature is on, or when the runtime is beside the
//! executable: with the exports but no `D3D12\D3D12Core.dll`, device creation fails with
//! `D3D12_ERROR_INVALID_REDIST`. Without the exports the in-box runtime is used (Windows 11).
//! `cargo xtask agility` touches this file after installing (or removing) the runtime, so
//! cargo reruns this check.

use std::path::PathBuf;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    // OUT_DIR is <target>/<profile>/build/<package>-<hash>/out; the exe and
    // its D3D12\ folder are in <target>/<profile>/.
    let out = PathBuf::from(std::env::var_os("OUT_DIR").expect("cargo sets OUT_DIR"));
    let Some(profile) = out.ancestors().nth(3) else {
        return;
    };
    let core = profile.join("D3D12").join("D3D12Core.dll");
    let msvc = std::env::var("CARGO_CFG_TARGET_ENV").as_deref() == Ok("msvc");
    let diagnostics = std::env::var_os("CARGO_FEATURE_DIAGNOSTICS").is_some();
    if msvc && core.exists() {
        println!("cargo:rerun-if-changed={}", core.display());
    }
    if diagnostics || (msvc && core.exists()) {
        println!("cargo:rustc-link-arg-bins=/EXPORT:D3D12SDKVersion,DATA");
        println!("cargo:rustc-link-arg-bins=/EXPORT:D3D12SDKPath,DATA");
    }
}
