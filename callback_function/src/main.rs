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
