use rand::Rng;

pub struct GameMap {
    pub width: u32,
    pub height: u32,
    pub planet_locations: Vec<(u32, u32)>,
}

impl GameMap {
    pub fn new(width: u32, height: u32, planet_count: u32) -> GameMap {
        let mut planet_locations = Vec::new();
        let mut rng = rand::thread_rng();
        for _ in 0..planet_count {
            let x = rng.gen_range(0..width);
            let y = rng.gen_range(0..height);
            planet_locations.push((x, y));
        }
        GameMap {
            width,
            height,
            planet_locations,
        }
    }

    pub fn distance(p1: (u32, u32), p2: (u32, u32)) -> f64 {
        let x = (p1.0 as i32 - p2.0 as i32).pow(2);
        let y = (p1.1 as i32 - p2.1 as i32).pow(2);
        ((x + y) as f64).sqrt()
    }

    pub fn get_closest_planet(&self, player_pos: (u32, u32)) -> (u32, u32) {
        let mut closest_planet = self.planet_locations[0];
        let mut closest_dist = Self::distance(player_pos, closest_planet);

        for planet in &self.planet_locations {
            let dist = Self::distance(player_pos, *planet);
            if dist < closest_dist {
                closest_planet = *planet;
                closest_dist = dist;
            }
        }

        closest_planet
    }
}