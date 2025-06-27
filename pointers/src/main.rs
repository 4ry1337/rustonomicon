unsafe extern "C" {
    fn fill_data(buf: *mut u8, len: usize);
}

fn main() {
    let mut buffer = [0u8; 16];
    unsafe {
        fill_data(buffer.as_mut_ptr(), buffer.len());
        println!("Buffer: {:?}", std::str::from_utf8(&buffer).unwrap());
    }
}

// UB if:
// fill_data writes beyond len bytes.
// fill_data expects null-terminated but Rust doesn't guarantee it.
//
// from_utf8 assumes valid UTF-8; C might write invalid bytes → use from_utf8_lossy if unsure.
//
// Null-byte search before slicing ensures safety.
