//! Health
//! Defines the agent health

use std::fmt::{Display, Formatter};

const MAX_HEALTH: f32 = 100.0;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Health {
    pub amount: f32,
    pub max: f32,
}

impl Health {
    pub fn at(percent: f32) -> Self {
        let amount = MAX_HEALTH * (percent / 100.0);
        Self {
            amount,
            max: MAX_HEALTH,
        }
    }
}

impl Default for Health {
    fn default() -> Self {
        Self {
            amount: MAX_HEALTH,
            max: MAX_HEALTH,
        }
    }
}

impl Display for Health {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}%", self.amount / self.max * 100.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn health_from_float() {
        let health = Health::at(10.23);
        dbg!(&health);
        assert_eq!(health.amount, 10.23);
        assert_eq!(health.max, MAX_HEALTH);
    }

    #[test]
    fn health_from_float_with_precision() {
        let health = Health::at(50.55);
        dbg!(&health);
        assert_eq!(health.amount, 50.550003);
        assert_eq!(health.max, MAX_HEALTH);
    }
}
