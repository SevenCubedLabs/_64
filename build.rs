fn main() {
    if !cfg!(feature = "edit") {
        println!("cargo:rustc-link-arg-bin=_64=-nostartfiles");
    }
}
