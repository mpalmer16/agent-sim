//! AgentKind
//! Defines the agent kind

use std::fmt::{Display, Formatter};

use rand::{distributions::Standard, prelude::Distribution};

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum AgentKind {
    Human,
    Monster,
    Plant,
}

impl Distribution<AgentKind> for Standard {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> AgentKind {
        match rng.gen_range(0..=2) {
            0 => AgentKind::Human,
            1 => AgentKind::Monster,
            _ => AgentKind::Plant,
        }
    }
}

impl Display for AgentKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
