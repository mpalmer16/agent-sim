use agent_lib::{agent::{
    health::Health,
    kind::AgentKind::{Human, Monster, Plant},
}, Agent};

use crate::Behavior;

#[derive(Clone)]
pub struct NormalBehavior;

impl Behavior for NormalBehavior {

    fn behave(&self, agent: &Agent, others: Vec<&Agent>) -> (Agent, (isize, isize)) {
        let updated_agent = self.interact_behavior(&self.progress_behavior(agent), others);
        (updated_agent, self.agent_move(&updated_agent))
    }
}

impl NormalBehavior {
    fn progress_behavior(&self, agent: &Agent) -> Agent {
        let new_health = Health {
            amount: agent.health.amount - 1.0 * agent.level.minor as f32,
            max: agent.health.max,
        };
        Agent {
            level: agent.level,
            health: new_health,
            kind: agent.kind,
            is_alive: new_health.amount > 0.0,
        }
    }

    fn agent_move(&self, agent: &Agent) -> (isize, isize) {
        match agent.kind {
            Human => (1, 1),
            Monster => (0, -2),
            Plant => (0, 0),
        }
    }

    fn interact_behavior(&self, a: &Agent, others: Vec<&Agent>) -> Agent {
        let mut agent = *a;
        others
            .iter()
            .filter(|&&other| agent != *other)
            .copied()
            .collect::<Vec<&Agent>>()
            .iter()
            .for_each(|other| {
                if other.level >= agent.level {
                    let diff = agent.level.major - other.level.major;
                    if diff < 0 {
                        if agent.level.major > 0 {
                            agent.level.major -= 1;
                        } else {
                            agent.is_alive = false;
                        }
                    } else if diff >= 0 {
                        if agent.level.minor == 9 {
                            agent.level.major += 1;
                            agent.level.minor = 0;
                        } else {
                            agent.level.minor += 1;
                        }
                    }
                } else {
                    agent.level.major += 1;
                }
            });
        agent
    }
}
