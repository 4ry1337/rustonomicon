// Disables name mangling, required for C linkage
#[unsafe(no_mangle)]
pub extern "C" fn multiply(x: i32, y: i32) -> i32 {
    x * y
}
