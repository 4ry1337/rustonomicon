// Import C function from external source
unsafe extern "C" {
    fn square_root(x: f64) -> f64;
}

fn main() {
    let x = 49.0;

    unsafe {
        let result = square_root(x); // ✅ FFI call must be inside unsafe block
        println!("sqrt({}) = {}", x, result);
    }
}

// UB if square_root is not linked properly (e.g. missing .so/.a, or wrong signature).
//
// Calling outside of unsafe block would be a compiler error (safety contract).
//
// Safe if linked and the C function has matching ABI and type.
