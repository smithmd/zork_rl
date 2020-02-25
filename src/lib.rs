extern crate rand;

use rand::Rng;
use std::error::Error;

pub struct Dungeon {
    floor_count: i16,
}

impl Dungeon {
    pub fn new() -> Dungeon {
        let mut rng = rand::thread_rng();
        let floor_count = rng.gen::<i16>();
        Dungeon { floor_count }
    }
}

pub struct Floor {}

pub fn run() -> Result<(), Box<dyn Error>> {
    Ok(())
}