//! Main entry point for the CLI RPG game.
//! Orchestrates the game loop and modular interaction.

mod character;
mod enemy;
mod battle;
mod ui;
mod utils;

use character::Character;
use ui::{print_ascii_banner, clear_screen};
use enemy::encounter_enemy;
use battle::{battle, win_handler, lose_handler};

/// Initializes the game, creates the player character, and starts the game loop.
fn main() {
    clear_screen();
    print_ascii_banner(1);
    println!("Alpha V1.0.1");
    
    let mut player = Character::new_character();
    game_loop(&mut player);
}

/// The continuous loop that generates enemy encounters and manages game progression.
fn game_loop(player: &mut Character) {
    loop {
        let mut enemy = encounter_enemy(player);
        println!("A level {} {} Appears!", enemy.level, enemy.name);
        
        let game_won = battle(player, &mut enemy);
        
        if game_won {
            if win_handler(player, &enemy) {
                continue;
            }
        } else if lose_handler(player, &enemy) {
            continue;
        }
    }
}
