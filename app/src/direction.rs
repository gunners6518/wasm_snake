#[derive(Debug, PartialEq, Copy, Clone)]

//型定義
pub enum Direction {
  Up,
  Down,
  Left,
  Right,
}

impl Direction {
  pub fn opposite(self, other: Direction) -> bool {
    self == Direction::Up && other == Direction::Down
      || self == Direction::Down && other == Direction::Up
      || self == Direction::Left && other == Direction::Right
      || self == Direction::Right && other == Direction::Left
  }
}
