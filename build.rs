fn main() {
    #[cfg(target_os = "linux")]
    linux();
}

fn linux() {
    println!("cargo:rustc-link-arg=-nostartfiles");
    println!("cargo:rustc-link-arg=-Tscript.ld");
    println!("cargo:rerun-if-changed=./script.ld");
}
