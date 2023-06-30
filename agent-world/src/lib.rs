use std::{
    collections::HashMap,
    fmt::{Debug, Display, Formatter},
};

use agent_behavior::Behavior;
use agent_lib::{Agent, agent::{position::Position, kind::AgentKind}};
use rand::Rng;
use uuid::Uuid;

const WORLD_SIZE: usize = 25;

#[derive(Clone)]
pub struct World<T> {
    locations: HashMap<Uuid, Position>,
    agents: HashMap<Uuid, Agent>,
    behavior: T,
}

impl <T: Behavior + Clone> World<T>{
    pub fn fill(agent_count: usize, behavior: T) -> Self {
        let mut rng = rand::thread_rng();
        let mut locations = HashMap::new();
        let mut agents = HashMap::new();

        for _ in 0..agent_count {
            let x = rng.gen_range(0..WORLD_SIZE);
            let y = rng.gen_range(0..WORLD_SIZE);
            let id = Uuid::new_v4();
            let kind: AgentKind = rng.gen();
            agents.insert(id, Agent::new(kind));
            locations.insert(id, Position::new((x, y)));
        }
        Self { locations, agents, behavior }
    }

    pub fn get_agents_at(&self, pos_to_find: &Position) -> Vec<&Agent> {
        let at_pos = self
            .locations
            .iter()
            .filter(|(_, &p)| p == *pos_to_find)
            .map(|(id, _)| id)
            .collect::<Vec<_>>();

        at_pos
            .iter()
            .map(|id| self.agents.get(id).expect("agent not found!"))
            .collect()
    }

    pub fn get_agent_position(&self, id: &Uuid) -> Option<&Position> {
        self.locations.get(id)
    }

    pub fn tick(&mut self) {
        self.behave();
        self.bring_out_your_dead();
    }

    fn bring_out_your_dead(&mut self) {
        let agents_clone = self.agents.clone();
        let locations_clone = self.locations.clone();

        let dead_agents = agents_clone
            .iter()
            .filter(|(_, agent)| {
                !agent.is_alive || (agent.level.major == 0 && agent.level.minor == 0)
            })
            .map(|(id, _)| id)
            .collect::<Vec<_>>();
        self.locations = locations_clone
            .iter()
            .filter(|(id, _)| !dead_agents.contains(id))
            .map(|(id, pos)| (*id, *pos))
            .collect();
        self.agents = agents_clone
            .iter()
            .filter(|(id, _)| !dead_agents.contains(id))
            .map(|(id, agent)| (*id, *agent))
            .collect();
    }

    fn behave(&mut self) {
        let mut updated_agents = vec![];
        self.agents.iter().for_each(|(id, agent)| {

            let pos = self.locations.get(id).expect("cant find agent location!");
            let others = self.get_agents_at(pos);
            let (new_agent, new_pos) = self.behavior.behave(agent, others);

            let px = match pos.x.checked_add_signed(new_pos.0) {
                Some(x) => x % WORLD_SIZE,
                None => 0,
            };
            let py = match pos.y.checked_add_signed(new_pos.1) {
                Some(y) => y % WORLD_SIZE,
                None => 0,
            };
            
            updated_agents.push((*id, new_agent, Position::new((px, py))));
        });

        for (id, agent, new_position) in updated_agents {
            self.agents.insert(id, agent);
            self.locations.insert(id, new_position);
        }
    }
}

impl <T: Behavior + Clone> Display for World<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut grid: Vec<Vec<char>> = vec![vec![' '; WORLD_SIZE]; WORLD_SIZE];
        for (_, pos) in self.locations.iter() {
            if grid[pos.y][pos.x] == '0' {
                grid[pos.y][pos.x] = 'X';
            } else {
                grid[pos.y][pos.x] = '0';
            }
        }

        for row in grid {
            for c in row {
                write!(f, " {} ", c)?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

impl <T: Behavior + Clone> Debug for World<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut grid: Vec<Vec<char>> = vec![vec![' '; WORLD_SIZE]; WORLD_SIZE];
        for (_, pos) in self.locations.iter() {
            if grid[pos.y][pos.x] == '0' {
                grid[pos.y][pos.x] = 'X';
            } else {
                grid[pos.y][pos.x] = '0';
            }
        }

        for row in grid {
            for c in row {
                write!(f, " {} ", c)?;
            }
            writeln!(f)?;
        }
        for (id, agent) in self.agents.iter() {
            writeln!(f, "{} at {}", agent, self.get_agent_position(id).unwrap())?;
        }

        Ok(())
    }
}
