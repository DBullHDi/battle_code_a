//function to capture inputs from keyboard

use std::io;

fn capture_keyboard_input() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input
}