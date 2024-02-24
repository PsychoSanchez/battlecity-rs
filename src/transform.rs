#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LookDirection {
    Up,
    Down,
    Left,
    Right,
}

impl LookDirection {
    pub fn position_from(&self, position: &[i32; 2]) -> [i32; 2] {
        let [x, y] = *position;

        match self {
            LookDirection::Up => [x, y - 1],
            LookDirection::Down => [x, y + 1],
            LookDirection::Left => [x - 1, y],
            LookDirection::Right => [x + 1, y],
        }
    }
}
