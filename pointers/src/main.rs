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
