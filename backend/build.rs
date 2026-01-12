use std::{env, fs, path::{Path, PathBuf}, process::Command};

pub fn main() {
    println!();
    println!("cargo:warning=[build.rs] running build script");

    println!("cargo:rerun-if-changed=../frontend/package.json");
    println!("cargo:rerun-if-changed=../frontend/package-lock.json");
    println!("cargo:rerun-if-changed=../frontend/vite.config.ts");
    println!("cargo:rerun-if-changed=../frontend/tsconfig.json");
    println!("cargo:rerun-if-changed=../frontend/src");
    println!("cargo:rerun-if-changed=../frontend/public");

    let frontend_dir = Path::new("../frontend");

    let status = Command::new("npm")
        .args(["run", "build"])
        .current_dir(frontend_dir)
        .status()
        .unwrap_or_else(|e| {
            panic!(
                "failed to run `npm run build` in {frontend_dir:?}: {e}\n\
                 (Is Node/npm installed and on PATH?)"
            )
        });

    if !status.success() {
        panic!("frontend build failed");
    }

    let crate_root = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let dist_src = frontend_dir.join("dist");
    let build_dst = crate_root.join("build");

    if build_dst.exists() {
        fs::remove_dir_all(&build_dst).expect("failed to clear ./build");
    }
    copy_dir_all(&dist_src, &build_dst).expect("failed to copy dist -> ./build");

    println!("cargo:warning=[build.rs] frontend copied to {}", build_dst.display());
}

fn copy_dir_all(src: &Path, dst: &Path) -> std::io::Result<()> {
    fs::create_dir_all(dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        let from = entry.path();
        let to = dst.join(entry.file_name());
        if ty.is_dir() {
            copy_dir_all(&from, &to)?;
        } else {
            fs::copy(&from, &to)?;
        }
    }
    Ok(())
}