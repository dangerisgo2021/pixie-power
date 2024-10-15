#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
    None,
}

impl Direction {
    pub fn as_offset(self) -> (i32, i32) {
        let (x, y) = match self {
            Direction::Up => (0, 1),
            Direction::Right => (1, 0),
            Direction::Down => (0, -1),
            Direction::Left => (-1, 0),
            Direction::None => (0, 0),
        };
        (x, y)
    }
}