use crate::ship::Ship;
use crate::game_map::GameMap;

pub struct Player {
    pub ships: Vec<Ship>,

}

impl Player {
    pub fn new(game_map: &GameMap, num_ship: usize) -> Player {
        let mut ships = Vec::new();
        for _ in 0..num_ship {
            ships.push(Ship::new(&game_map));
        }
        Player {
            ships,
        }
    }

}