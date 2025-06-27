unsafe extern "C" {
    fn square_root(x: f64) -> f64;
}

fn main() {
    let x = 49.0;
    unsafe {
        let result = square_root(x);
        println!("sqrt({}) = {}", x, result);
    }
}
