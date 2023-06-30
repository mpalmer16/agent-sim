//! Position
//! Defines the agent position

use std::fmt::{Display, Formatter};

pub type Pos = (usize, usize);

#[derive(PartialEq, Eq, Hash, Debug, Copy, Clone)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

impl Position {
    pub fn new(pos: Pos) -> Self {
        Self { x: pos.0, y: pos.1 }
    }
}

impl Display for Position {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_default_agent() {
        let pos = Position::new((1, 2));

        assert!(pos == Position { x: 1, y: 2 });
    }
}
