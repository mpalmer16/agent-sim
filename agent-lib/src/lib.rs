//! Agent Lib

pub mod agent;

use std::fmt::{Display, Formatter};

use agent::{level::Level, health::Health, kind::AgentKind};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Agent {
    pub level: Level,
    pub health: Health,
    pub kind: AgentKind,
    pub is_alive: bool,
}

impl Agent {
    pub fn new(kind: AgentKind) -> Self {
        Self {
            level: Level::default(),
            health: Health::default(),
            kind,
            is_alive: true,
        }
    }
}

impl Display for Agent {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "level: {}, health: {}, kind: {}",
            self.level, self.health, self.kind
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_default_agent() {
        let agent = Agent::new(AgentKind::Human);

        assert!(agent.level == Level { major: 1, minor: 0 });
        assert!(
            agent.health
                == Health {
                    amount: 100.0,
                    max: 100.0
                }
        );
        assert!(agent.kind == AgentKind::Human);
    }
}
