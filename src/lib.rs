extern crate rand;

use rand::Rng;
use std::error::Error;

pub struct Floor {
    width: u16,
    height: u16,
}


pub struct Dungeon {
    floor_count: u16,
    floors: Vec<Floor>,
}

impl Dungeon {
    pub fn new() -> Dungeon {
        let mut rng = rand::thread_rng();
        let floor_count = rng.gen_range(1, 5);

        // build floor vector
        let mut floors = Vec::new();
        for f in 0..floor_count {
            println!("Making floor {}", f+1);
            let width = rng.gen_range(1, 10);
            let height = rng.gen_range(1, 10);
            floors.push(Floor { width, height });
        }

        Dungeon { floor_count, floors }
    }

    pub fn print_floor(&mut self, number: usize) {
        // get floor at {number}
        println!("Floor {}", number+1);
        let floor = &self.floors[number];
        println!("\t{}w x {}h", floor.width, floor.height);
    }
}

pub fn run() -> Result<(), Box<dyn Error>> {
    let mut d: Dungeon = Dungeon::new();
    for floor_num in 0..d.floor_count{
        d.print_floor(usize::from(floor_num));
    }
    Ok(())
}