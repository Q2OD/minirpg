use std::io::{self, Write};
use rand::Rng;

/// Reads a line from standard input, trims whitespace, and returns it as a String.
pub fn input() -> String {
    let mut line = String::new();
    io::stdin()
        .read_line(&mut line)
        .expect("Failed to read line");
    line.trim().to_string()
}

/// Generates a random integer between min and max (inclusive) and prints a message describing the roll.
pub fn roll_stat(stat_name: &str, min: i32, max: i32) -> i32 {
    let value = rand::thread_rng().gen_range(min..=max);
    println!(
        "Rolling {} ({}-{})... you got {}!",
        stat_name, min, max, value
    );
    value
}

/// Generates a random stat value that scales based on the player's current level.
pub fn roll_level_stat(player: &crate::character::Character, stat_name: &str, min: i32, max: i32) -> i32 {
    // Stat scaling factor: 5% increase per level
    let factor = 1.0 + (player.level as f32 * 0.05);
    let min_factored = (min as f32 * factor) as i32;
    let value = rand::thread_rng().gen_range(min_factored..=max);
    println!(
        "Rolling {} ({}-{})... you got {}!",
        stat_name, min, max, value
    );
    value
}
