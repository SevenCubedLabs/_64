fn main() {
    println!("cargo:rerun-if-changed=bindings.h");

    let bindings = bindgen::builder()
        .prepend_enum_name(false)
        .derive_debug(false)
        .derive_eq(false)
        .header("bindings.h")
        .blocklist_item("FP_.*")
        .use_core()
        .ctypes_prefix("core::ffi")
        .generate()
        .unwrap();

    let out_path = std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
