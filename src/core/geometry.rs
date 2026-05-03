#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GridPosition {
    pub x: i32,
    pub y: i32,
}

pub(crate) fn distance(a: GridPosition, b: GridPosition) -> i32 {
    (a.x - b.x).abs() + (a.y - b.y).abs()
}
