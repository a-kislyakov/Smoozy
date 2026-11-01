use std::{env, path::PathBuf, process::Command};

fn main() {
    // About and the notices — Desktop app and delivery, added 2026-10-27.
    // The generated text belongs to this target, and is compiled into About.
    let desktop = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").unwrap());
    let app = desktop.parent().unwrap();
    let out = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    let target = env::var("TARGET").unwrap();
    let profile_parent = out.ancestors().nth(4).expect("Cargo's build output folder");
    let target_dir = if profile_parent.file_name().and_then(|s| s.to_str()) == Some(target.as_str())
    {
        profile_parent.parent().unwrap()
    } else {
        profile_parent
    };
    let generator = app.join("Release/notices.py");
    let generated = app.join("Release/generated-notices");
    let result = Command::new(if cfg!(windows) { "python" } else { "python3" })
        .arg(&generator)
        .arg("--target")
        .arg(&target)
        .arg("--target-dir")
        .arg(target_dir)
        .arg("--into")
        .arg(&out)
        .status()
        .expect("Python 3 is required to build the bundled notices");
    assert!(
        result.success(),
        "the third-party notices could not be generated"
    );
    std::fs::create_dir_all(&generated).unwrap();
    for file in [
        "COPYING",
        "THIRD-PARTY-LICENSES.txt",
        "THIRD-PARTY-LICENSES-server.txt",
        "copyright",
        "packages.json",
    ] {
        std::fs::copy(out.join(file), generated.join(file)).unwrap();
    }
    println!("cargo:rerun-if-changed={}", generator.display());
    println!(
        "cargo:rerun-if-changed={}",
        app.join("Release/licence-supplements").display()
    );
    println!(
        "cargo:rerun-if-changed={}",
        app.parent().unwrap().join("COPYING").display()
    );
    println!(
        "cargo:rerun-if-changed={}",
        app.join("Cargo.lock").display()
    );
    println!(
        "cargo:rerun-if-changed={}",
        app.join("Cargo.toml").display()
    );
    for entry in std::fs::read_dir(app).unwrap().flatten() {
        let manifest = entry.path().join("Cargo.toml");
        if manifest.is_file() {
            println!("cargo:rerun-if-changed={}", manifest.display());
        }
    }
    println!("cargo:rerun-if-changed=src/the_address_opener.rs");
    println!("cargo:rerun-if-changed=src/surface/menus.rs");
    tauri_build::build()
}
