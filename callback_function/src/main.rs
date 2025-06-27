unsafe extern "C" fn rust_callback(x: i32) {
    println!("Callback called with {}", x);
}

unsafe extern "C" {
    fn call_callback(cb: unsafe extern "C" fn(i32));
}

fn main() {
    unsafe {
        call_callback(rust_callback);
    }
}

// UB if:
// Rust passes an invalid or null function pointer.
// C calls back with wrong signature (e.g. passing float instead of int).
//
// Matching extern "C" on both sides ensures ABI compatibility.
//
// Rust function must not panic — unwinding across FFI boundary is UB.
