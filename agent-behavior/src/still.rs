use crate::Behavior;

use agent_lib::Agent;



#[derive(Clone)]
pub struct StillBehavior;

impl Behavior for StillBehavior {
    fn behave(&self, agent: &Agent, _others: Vec<&Agent>) -> (Agent, (isize, isize)) {
        (*agent, (0, 0))
    }
}