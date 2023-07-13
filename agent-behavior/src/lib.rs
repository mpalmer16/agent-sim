use agent_lib::Agent;

pub mod normal;
pub mod still;
pub mod agressive;


pub trait Behavior {
    fn behave(&self, agent: &Agent, others: Vec<&Agent>) -> (Agent, (isize, isize));
}
