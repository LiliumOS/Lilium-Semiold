fn main() {
    println!(
        "cargo:rustc-cdylib-link-arg=-T{}/../ld/{}-modules.ld",
        std::env::var("CARGO_MANIFEST_DIR").unwrap(),
        std::env::var("TARGET").unwrap()
    );
    println!(
        "cargo:rustc-cdylib-link-arg=-Wl,-soname,{}",
        std::env!("kernel_name")
    );
    print!("cargo:rustc-cdylin-link-arg=-Wl,-e,_start");
}
