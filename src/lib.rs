extern crate rand;

use rand::Rng;
use std::error::Error;

pub struct Room {
    is_usable: bool
}

pub struct Door {
    is_open: bool,
    is_blocked: bool,
    is_locked: bool
}

pub struct Floor {
    width: u16,
    height: u16,
    grid: Vec<Room>
}

impl Floor {
    pub fn new(width: u16, height: u16) -> Floor {
        // build grid
        let mut grid: Vec<Room> = Vec::new();
        let mut rng = rand::thread_rng();

        for _r in 0..width {
            for _c in 0..height {
                grid.push(Room { is_usable: rng.gen_bool(0.5) });
            }
        }

        Floor { width, height, grid }
    }

    pub fn print_floor(&self) {
        for r in 0..self.width {
            for c in 0..self.height {
                if self.grid[(r*c) as usize].is_usable {
                    print!("R ");
                } else {
                    print!("  ");
                }
            }
            print!("\n");
        }
    }
}

pub struct Dungeon {
    floor_count: u16,
    floors: Vec<Floor>,
}

impl Dungeon {
    pub fn new() -> Dungeon {
        let mut rng = rand::thread_rng();
        let floor_count = rng.gen_range(3, 7);

        // build floor vector
        let mut floors = Vec::new();
        for f in 0..floor_count {
            println!("Making floor {}", f + 1);
            floors.push(Floor::new(
                rng.gen_range(3, 10),
                rng.gen_range(3, 10))
            );
        }

        Dungeon { floor_count, floors }
    }

    pub fn print_dungeon(&self) {
        for floor_num in 0..self.floor_count {
            // get floor at {floor_num}
            println!("Floor {}", floor_num + 1);
            let floor = &self.floors[floor_num as usize];
            println!("\t{}w x {}h", floor.width, floor.height);
            floor.print_floor();
        }
    }
}

pub fn run() -> Result<(), Box<dyn Error>> {
    let dungeon: Dungeon = Dungeon::new();
    dungeon.print_dungeon();
    Ok(())
}