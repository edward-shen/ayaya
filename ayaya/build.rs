fn main() {
    #[cfg(target_os = "linux")]
    linux();
}

fn linux() {
    println!("cargo:rustc-link-arg=-nostartfiles");
    println!("cargo:rustc-link-arg=-T./ayaya/script.ld");
    println!("cargo:rerun-if-changed=./ayaya/script.ld");
}
