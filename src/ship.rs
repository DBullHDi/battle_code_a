use rand::Rng;

use crate::game_map::GameMap;

pub struct Ship {
    pub pos: (u32, u32),
    pub vertices: [(f32, f32); 3],
    pub fuel: i32,
    pub resources: i32
}

impl Ship {
    pub fn new(game_map: &GameMap) -> Ship {
        let mut rng = rand::thread_rng();
        let x = rng.gen_range(0..game_map.width);
        let y = rng.gen_range(0..game_map.height);
        
        Ship {
            pos: (x, y),
            fuel: 100,
            resources: 0,
            vertices: [(0.0, 0.0), (1.0, 0.0), (0.5, 1.0)]
        }
    }

    pub fn move_twoards(&mut self, target: (u32, u32), game_map: &GameMap) {
        let mut x = self.pos.0;
        let mut y = self.pos.1;
        if self.pos.0 < target.0 {
            x += 1;
        } else if self.pos.0 > target.0 {
            x -= 1;
        }

        if self.pos.1 < target.1 {
            y += 1;
        } else if self.pos.1 > target.1 {
            y -= 1;
        }
        self.pos = (x, y);
    }

    pub fn mine(&mut self, planet: (u32, u32)) {
        let dist = GameMap::distance(self.pos, planet);
        if dist > 1.0 {
            println!("Too far away to mine");
        } else {
            println!("Mining");
            self.resources += 1;
        }
    }
    

}

