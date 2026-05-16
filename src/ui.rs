use std::io::{self, Write};
use crate::character::Character;

/// Defines the visual style of the message box displayed during combat.
pub enum BoxType {
    Attack,
    Defend,
    Heal,
}

/// Prints a solid separator line to visually distinguish different turns in the console.
pub fn turn_separator() {
    println!("████████████████████████")
}

/// Clears the terminal screen and resets the cursor to the top-left position.
pub fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
    io::stdout().flush().unwrap()
}

/// Prints the current health status of both the player and the enemy.
pub fn print_status(player: &Character, enemy: &Character) {
    println!(
        "{}: {}/ {} HP",
        player.name, player.health, player.max_health
    );
    turn_separator();
    println!("{}: {}/ {} HP", enemy.name, enemy.health, enemy.max_health);
}

/// Prints a decorative box containing a combat event message (e.g., damage taken or healing).
pub fn print_box(attacker: &Character, defender: &Character, amount: i32, box_type: BoxType) {
    let message = match box_type {
        BoxType::Attack => {
            format!(
                "{} takes {} HP from {}.",
                defender.name, amount, attacker.name
            )
        }
        BoxType::Heal => {
            format!("{} heals {} HP.", attacker.name, amount)
        }
        BoxType::Defend => {
            format!("{} is blocking.", attacker.name)
        }
    };
    let padding: usize = 4;
    let width = message.len() + padding;
    println!("┌{}┐", "─".repeat(width));
    println!("│{:^width$}│", message, width = width);
    println!("└{}┘", "─".repeat(width));
}

/// Prints a specific ASCII art banner based on the provided ID.
/// 
/// IDs:
/// 1: Main Logo
/// 2: Victory
/// 3: Defeat
/// 4: Game Over
/// 5: Exit
/// 6: Win Screen
/// 7: New Encounter
pub fn print_ascii_banner(ascii: i32) {
    match ascii {
        1 => {
            println!(
    "                                                                                                                                                                                                                                                                                                                                 
▗▖ ▗▖▗▄▄▄▖▗▖    ▗▄▄▖ ▗▄▖ ▗▖  ▗▖▗▄▄▄▖    ▗▄▄▄▖▗▄▖     ▗▖  ▗▖▗▄▄▄▖▗▖  ▗▖▗▄▄▄▖    ▗▄▄▖ ▗▄▄▖  ▗▄▄▖
▐▌ ▐▌▐▌   ▐▌   ▐▌   ▐▌ ▐▌▐▛▚▞▜▌▐▌         █ ▐▌ ▐▌    ▐▛▚▞▜▌  █  ▐▛▚▖▐▌  █      ▐▌ ▐▌▐▌ ▐▌▐▌   
▐▌ ▐▌▐▛▀▀▘▐▌   ▐▌   ▐▌ ▐▌▐▌  ▐▌▐▛▀▀▘      █ ▐▌ ▐▌    ▐▌  ▐▌  █  ▐▌ ▝▜▌  █      ▐▛▀▚▖▐▛▀▘ ▐▌▝▜▌
▐▙█▟▌▐▙▄▄▖▐▙▄▄▖▝▚▄▄▖▝▚▄▞▘▐▌  ▐▌▐▙▄▄▖      █ ▝▚▄▞▘    ▐▌  ▐▌▗▄█▄▖▐▌  ▐▌▗▄█▄▖    ▐▌ ▐▌▐▌   ▝▚▄▞▘                                         
        "
        );
        }
        2 => {
            println!(
                "
▗▖  ▗▖▗▄▖ ▗▖ ▗▖    ▗▖ ▗▖▗▄▄▄▖▗▖  ▗▖
 ▝▚▞▘▐▌ ▐▌▐▌ ▐▌    ▐▌ ▐▌  █  ▐▛▚▖▐▌
  ▐▌ ▐▌ ▐▌▐▌ ▐▌    ▐▌ ▐▌  █  ▐▌ ▝▜▌
  ▐▌ ▝▚▄▞▘▝▚▄▞▘    ▐▙█▟▌▗▄█▄▖▐▌  ▐▌
        "
            );
        }
        3 => {
            println!(
    "
▗▖  ▗▖▗▄▖ ▗▖ ▗▖    ▗▖ ▗▖▗▄▄▄▖▗▄▄▖ ▗▄▄▄▖    ▗▄▄▄ ▗▄▄▄▖▗▄▄▄▖▗▄▄▄▖ ▗▄▖▗▄▄▄▖▗▄▄▄▖▗▄▄▄ 
 ▝▚▞▘▐▌ ▐▌▐▌ ▐▌    ▐▌ ▐▌▐▌   ▐▌ ▐▌▐▌       ▐▌  █▐▌   ▐▌   ▐▌   ▐▌ ▐▌ █  ▐▌   ▐▌  █
  ▐▌ ▐▌ ▐▌▐▌ ▐▌    ▐▌ ▐▌▐▛▀▀▘▐▛▀▚▖▐▛▀▀▘    ▐▌  █▐▛▀▀▘▐▛▀▀▘▐▛▀▀▘▐▛▀▜▌ █  ▐▛▀▀▘▐▌  █
  ▐▌ ▝▚▄▞▘▝▚▄▞▘    ▐▙█▟▌▐▙▄▄▖▐▌ ▐▌▐▙▄▄▖    ▐▙▄▄▀▐▙▄▄▖▐▌   ▐▙▄▄▖▐▌ ▐▌ █  ▐▙▄▄▖▐▙▄▄▀                                                                    
        "
        );
        }
        4 => {
            println!(
    "
 ▗▄▄▖ ▗▄▖ ▗▖  ▗▖▗▄▄▄▖     ▗▄▖ ▗▖  ▗▖▗▄▄▄▖▗▄▄▖     
▐▌   ▐▌ ▐▌▐▛▚▞▜▌▐▌       ▐▌ ▐▌▐▌  ▐▌▐▌   ▐▌ ▐▌    
▐▌▝▜▌▐▛▀▜▌▐▌  ▐▌▐▛▀▀▘    ▐▌ ▐▌▐▌  ▐▌▐▛▀▀▘▐▛▀▚▖    
▝▚▄▞▘▐▌ ▐▌▐▌  ▐▌▐▙▄▄▖    ▝▚▄▞▘ ▝▚▞▘ ▐▙▄▄▖▐▌ ▐▌                                                           
        "
        );
        }
        5 => {
            println!(
    "
▗▄▄▄▖▗▖ ▗▖ ▗▄▖ ▗▖  ▗▖▗▖ ▗▖    ▗▖  ▗▖▗▄▖ ▗▖ ▗▖    ▗▄▄▄▖ ▗▄▖ ▗▄▄▖     ▗▄▄▖ ▗▖    ▗▄▖▗▖  ▗▖▗▄▄▄▖▗▖  ▗▖ ▗▄▄▖
  █  ▐▌ ▐▌▐▌ ▐▌▐▛▚▖▐▌▐▌▗▞▘     ▝▚▞▘▐▌ ▐▌▐▌ ▐▌    ▐▌   ▐▌ ▐▌▐▌ ▐▌    ▐▌ ▐▌▐▌   ▐▌ ▐▌▝▚▞▘   █  ▐▛▚▖▐▌▐▌   
  █  ▐▛▀▜▌▐▛▀▜▌▐▌ ▝▜▌▐▛▚▖       ▐▌ ▐▌ ▐▌▐▌ ▐▌    ▐▛▀▀▘▐▌ ▐▌▐▛▀▚▖    ▐▛▀▘ ▐▌   ▐▛▀▜▌ ▐▌    █  ▐▌ ▝▜▌▐▌▝▜▌
  █  ▐▌ ▐▌▐▌ ▐▌▐▌  ▐▌▐▌ ▐▌      ▐▌ ▝▚▄▞▘▝▚▄▞▘    ▐▌   ▝▚▄▞▘▐▌ ▐▌    ▐▌   ▐▙▄▄▖▐▌ ▐▌ ▐▌  ▗▄█▄▖▐▌  ▐▌▝▚▄▞▘                                                      
        "
        );
        }
        6 => {
            println!(
    "
▗▄▄▖  ▗▄▖ ▗▖ ▗▖▗▖  ▗▖▗▄▄▄      ▗▄▄▖▗▖ ▗▖▗▖  ▗▖▗▖  ▗▖ ▗▄▖ ▗▄▄▖▗▖  ▗▖
▐▌ ▐▌▐▌ ▐▌▐▌ ▐▌▐▛▚▖▐▌▐▌  █    ▐▌   ▐▌ ▐▌▐▛▚▞▜▌▐▛▚▞▜▌▐▌ ▐▌▐▌ ▐▌▝▚▞▘ 
▐▛▀▚▖▐▌ ▐▌▐▌ ▐▌▐▌ ▝▜▌▐▌  █     ▝▀▚▖▐▌ ▐▌▐▌  ▐▌▐▌  ▐▌▐▛▀▜▌▐▛▀▚▖ ▐▌  
▐▌ ▐▌▝▚▄▞▘▝▚▄▞▘▐▌  ▐▌▐▙▄▄▀    ▗▄▄▞▘▝▚▄▞▘▐▌  ▐▌▐▌  ▐▌▐▌ ▐▌▐▌ ▐▌ ▐▌                                                
        "
        );
        }
        7 => {
            println!(
                "
▗▖  ▗▖▗▄▄▄▖▗▖ ▗▖    ▗▄▄▄▖▗▖  ▗▖▗▄▄▄▖▗▖  ▗▖▗▖  ▗▖
▐▛▚▖▐▌▐▌   ▐▌ ▐▌    ▐▌   ▐▛▚▖▐▌▐▌   ▐▛▚▞▜▌ ▝▚▞▘ 
▐▌ ▝▜▌▐▛▀▀▘▐▌ ▐▌    ▐▛▀▀▘▐▌ ▝▜▌▐▛▀▀▘▐▌  ▐▌  ▐▌  
▐▌  ▐▌▐▙▄▄▖▐▙█▟▌    ▐▙▄▄▖▐▌  ▐▌▐▙▄▄▖▐▌  ▐▌  ▐▌                  
        "
            );
        }
        _ => unreachable!(),
    }
}
