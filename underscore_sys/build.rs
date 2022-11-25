fn main() {
    println!("cargo:rerun-if-changed=c_deps.h");
    println!("cargo:rustc-link-lib=c");
    println!("cargo:rustc-link-lib=SDL2");
    println!("cargo:rustc-link-lib=GL");

    let underscore_sys = bindgen::builder()
        .prepend_enum_name(false)
        .derive_debug(false)
        .derive_eq(false)
        .header("c_deps.h")
        .clang_arg("-I/usr/include/SDL2/")
        .blocklist_item("FP_.*")
        .use_core()
        .ctypes_prefix("crate::underscore_sys::c_types")
        .generate()
        .unwrap();

    let out_path = std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap());
    underscore_sys
        .write_to_file(out_path.join("underscore_sys.rs"))
        .expect("Couldn't write underscore_sys!");
}