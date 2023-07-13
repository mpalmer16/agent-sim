use std::{thread, time::Duration};

use agent_behavior::{normal::NormalBehavior, agressive::AgressiveBehavior};
use agent_world::World;


fn main() {

    let mut active_world = World::fill(100, Box::new(NormalBehavior));
    let mut agressive_world = World::fill(100, Box::new(AgressiveBehavior));

    let mut n = 200;
    while n > 0 {
        clear_console();

        println!("{}", active_world);
        println!();
        println!("{}", agressive_world);

        active_world.tick();
        agressive_world.tick();

        thread::sleep(Duration::from_millis(100));
        n -= 1;
    }
}

fn clear_console() {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
}