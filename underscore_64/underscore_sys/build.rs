fn main() {
    println!("cargo:rerun-if-changed=src/bindings.h");
    println!("cargo:rustc-link-lib=c");
    println!("cargo:rustc-link-lib=SDL2");
    println!("cargo:rustc-link-lib=GL");

    let bindings = bindgen::builder()
        .prepend_enum_name(false)
        .derive_debug(false)
        .derive_eq(false)
        .header("src/bindings.h")
        .clang_arg("-I/usr/include/SDL2/")
        .blocklist_item("FP_.*")
        .use_core()
        .ctypes_prefix("crate::c_types")
        .generate()
        .unwrap();

    let out_path = std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
