//! Level
//! Defines the agent level

use std::fmt::{Display, Formatter};

#[derive(Debug, PartialEq, PartialOrd, Copy, Clone)]
pub struct Level {
    pub major: i32,
    pub minor: i32,
}

impl Level {
    pub fn at(level: f32) -> Self {
        let major = level as i32;
        let minor = ((level * 100.0).ceil() % 100.0) as i32;
        Self { major, minor }
    }
}

impl Default for Level {
    fn default() -> Self {
        Self { major: 1, minor: 0 }
    }
}

impl Display for Level {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}", self.major, self.minor)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn level_from_float() {
        let level = Level::at(10.23);
        assert_eq!(level.major, 10);
        assert_eq!(level.minor, 23);
    }
}
