fn main() {
    if cfg!(feature = "minsize") {
        println!("cargo:rustc-link-arg-bin=_64=-nostartfiles");
    }
}
