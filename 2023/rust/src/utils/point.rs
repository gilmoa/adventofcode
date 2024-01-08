#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

pub fn distance_manhattan(a: Point, b: Point) -> u32 {
    a.x.abs_diff(b.x) + a.y.abs_diff(b.y)
}
