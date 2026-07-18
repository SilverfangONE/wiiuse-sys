use std::env;
use std::fs;
use std::path::PathBuf;

fn main() {
    let force_static = env::var("CARGO_FEATURE_STATIC").is_ok();

    println!("cargo:rustc-link-lib=wiiuse");

    if !force_static && pkg_config::probe_library("wiiuse").is_ok() {
        generate_bindings();
        return;
    }

    println!("cargo:warning=wiiuse not found on system. Compile from sources...");

    let src_dir = PathBuf::from("wiiuse-src");
    let mut build = cc::Build::new();

    build.include(&src_dir);

    if let Ok(entries) = fs::read_dir(&src_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_file() && path.extension().map_or(false, |ext| ext == "c") {
                build.file(path);
            }
        }
    }

    // platform depended code for bluetooth stack on os-level
    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap();
    if target_os == "windows" {
        build.file(src_dir.join("src/win32/win_hid.c"));
        println!("cargo:rustc-link-lib=setupapi");
        println!("cargo:rustc-link-lib=hid");
    } else if target_os == "linux" {
        build.file(src_dir.join("src/os_nix.c"));
        println!("cargo:rustc-link-lib=bluetooth");
    }

    build.compile("wiiuse");

    generate_bindings();
}

fn generate_bindings() {
    println!("cargo:rerun-if-changed=wrapper.h");

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .clang_arg("-Iwiiuse-src/src")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("coulnd't generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    let bindings_code = bindings.to_string();
    let fixed_bindings = bindings_code
        .replace("extern \"C\"", "unsafe extern \"C\"")
        .replace("unsafe unsafe extern \"C\"", "unsafe extern \"C\"");
    fs::write(out_path.join("bindings.rs"), fixed_bindings).expect("couldn't write bindings");
}
