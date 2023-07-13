use crate::Behavior;

use agent_lib::{Agent, agent::kind::AgentKind::{Human, Monster, Plant}};

#[derive(Clone)]
pub struct AgressiveBehavior;

impl Behavior for AgressiveBehavior {
    fn behave(&self, agent: &Agent, others: Vec<&Agent>) -> (Agent, (isize, isize)) {

        let mut agent = *agent;

        others
            .iter()
            .filter(|&&other| agent != *other)
            .copied()
            .collect::<Vec<&Agent>>()
            .iter()
            .for_each(|other| {
                match other.kind {
                    Human => {
                        let diff = agent.level.major - other.level.major;
                        match diff {
                            0 => (),
                            1 => agent.level.minor += 1,
                            _ => agent.level.minor -= 1,
                        }
                    }
                    Monster => {
                        agent.is_alive = false;
                    },
                    Plant => {
                        agent.health.amount += 10.0;
                    },
                }
            });

        let movement = agressive_movement(&agent);

        (agent, movement)
    }
}


fn agressive_movement(agent: &Agent) -> (isize, isize) {
    match agent.kind {
        Human => (5, -5),
        Monster => (10, 0),
        Plant => (0, 0),
    }
}