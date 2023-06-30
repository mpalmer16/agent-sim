use std::{thread, time::Duration};

use agent_behavior::{normal::NormalBehavior, still::StillBehavior};
use agent_world::World;


fn main() {
    let mut active_world = World::fill(100, NormalBehavior);
    let mut still_world = World::fill(100, StillBehavior);

    let mut n = 20;
    while n > 0 {
        clear_console();

        println!("{}", active_world);
        println!();
        println!("{}", still_world);

        active_world.tick();
        still_world.tick();

        thread::sleep(Duration::from_millis(100));
        n -= 1;
    }
}

fn clear_console() {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
}
