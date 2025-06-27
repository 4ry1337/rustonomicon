#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

unsafe extern "C" {
    fn print_point(p: Point);
}

fn main() {
    let pt = Point { x: 10, y: 20 };
    unsafe {
        print_point(pt);
    }
}
