fn main() {
    println!("cargo:rustc-link-lib=libdiscovery"); // the library name
    println!("cargo:rustc-link-search=native=../discovery_module"); // the library path
}