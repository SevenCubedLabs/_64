fn main() {
    #[cfg(not(feature = "std"))]
    println!("cargo:rustc-link-arg-bin=_64=-nostartfiles");
}
