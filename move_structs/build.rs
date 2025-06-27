fn main() {
    cc::Build::new().file("c_src/c_code.c").compile("c_code"); // creates lib (e.g., libc_code.a)
}
