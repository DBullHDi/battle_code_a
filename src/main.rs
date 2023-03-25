mod game_map;
mod player;
mod ship;

use std::io;
use std::io::Write;

use game_map::GameMap;
use player::Player;

pub fn main() {
    let game_map_instance: GameMap = GameMap::new(100, 100, 10);
    let mut player: Player = player::Player::new(&game_map_instance, 5);


    loop {
        print!("Enter a command: ");
        io::stdout().flush().unwrap();

        let mut input: String = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let action = input.trim();

        match action {
            "quit" => break,
            "help" => {
                println!("Commands: map, player, move, attack, quit");
            },
            "map" => {
                println!("Game map size: {}x{}", game_map_instance.width, game_map_instance.height);
                println!("Planets: {}", game_map_instance.planet_locations.len());
                println!("Planet locations: {:?}", game_map_instance.planet_locations);

                for ship in &player.ships {
                    println!("Ship position: {:?}", ship.pos);
                }
            },
            "player" => {
                for ship in &player.ships {
                    let closest_planet = game_map_instance.get_closest_planet(ship.pos);
                    println!("Ship position: {:?}", ship.pos);
                    println!("Closest planet to player: {:?}", closest_planet);
                    println!("Distance to closest planet: {}", GameMap::distance(ship.pos, closest_planet));
                    println!("Ship resources: {}", ship.resources);   
                }
                
                
                       
            },
            "move" => {
                for ship in &mut player.ships {
                    let closest_planet = game_map_instance.get_closest_planet(ship.pos);
                    ship.move_twoards(closest_planet, &game_map_instance);
                    println!("player position: {:?}", ship.pos);
                }
            },
            "mine" => {
                for ship in &mut player.ships {
                    let closest_planet = game_map_instance.get_closest_planet(ship.pos);
                    ship.mine(closest_planet);
                }
            },
            "attack" => {
                println!("Attacking not implemented yet");
            },
            _ => println!("Unknown command: {}", action),
        }

    }
}

