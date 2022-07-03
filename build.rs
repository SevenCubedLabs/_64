fn main() {
    println!("cargo:rerun-if-changed=bindings.h");
    println!("cargo:rustc-link-lib=c");
    println!("cargo:rustc-link-lib=SDL2");
    println!("cargo:rustc-link-lib=GL");
    if cfg!(feature = "minsize") {
        println!("cargo:rustc-link-arg-bin=_64=-nostartfiles");
    }

    let bindings = bindgen::builder()
        .prepend_enum_name(false)
        .derive_debug(false)
        .derive_eq(false)
        .header("bindings.h")
        .blocklist_item("FP_.*")
        .use_core()
        .ctypes_prefix("crate::sys::c_types")
        .generate()
        .unwrap();

    let out_path = std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
