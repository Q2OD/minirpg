use std::process::exit;
use rand::Rng;
use crate::character::Character;
use crate::ui::{print_ascii_banner, print_status, print_box, BoxType, turn_separator, clear_screen};
use crate::utils::input;

/// Represents the possible actions a player can take during their turn in battle.
pub enum BattleAction {
    Attack,
    Defend,
    Heal,
    Exit,
}

/// Represents the actions available in the post-battle menu.
pub enum MenuAction {
    Exit,
    PlayAgain,
}

/// Manages the core combat loop between the player and an enemy.
/// Returns `true` if the player wins, `false` if the player loses.
pub fn battle(player: &mut Character, enemy: &mut Character) -> bool {
    loop {
        // Check win/loss conditions
        if !player.is_alive() {
            print_ascii_banner(3);
            return false;
        }
        if !enemy.is_alive() {
            print_ascii_banner(2);
            return true;
        }

        turn_separator();
        print_status(player, enemy);
        turn_separator();

        // Player Turn
        let player_action = get_player_action();
        let enemy_action = get_enemy_action(enemy);
        apply_action(player, enemy, player_action);

        if !enemy.is_alive() {
            print_ascii_banner(2);
            return true;
        }

        turn_separator();

        // Enemy Turn
        apply_action(enemy, player, enemy_action);

        if !player.is_alive() {
            print_ascii_banner(3);
            return false;
        }

        // Cleanup turn states
        reset_blocking(player, enemy);
    }
}

/// Prompts the player to select a battle action from the menu.
pub fn get_player_action() -> BattleAction {
    println!("Choose your action: \n 1) Attack \n 2) Defend \n 3) Heal \n 4) Exit");
    let choice = input();
    match choice.as_str() {
        "1" => BattleAction::Attack,
        "2" => BattleAction::Defend,
        "3" => BattleAction::Heal,
        "4" => BattleAction::Exit,
        _ => {
            println!("Invalid Choice, try again.");
            get_player_action()
        }
    }
}

/// Prompts the player to decide whether to play again or exit after a battle.
pub fn get_play_again_action() -> MenuAction {
    println!("Play Again? \n y = continue \n n = exit gracefully \n");
    let action = input();
    match action.as_str() {
        "y" => MenuAction::PlayAgain,
        "n" => MenuAction::Exit,
        _ => {
            println!("Invalid Choice, try again.");
            get_play_again_action()
        }
    }
}

/// Determines the enemy's next move based on their current health.
pub fn get_enemy_action(enemy: &Character) -> BattleAction {
    // Enemy is more likely to heal if low on health
    let max_action: i32 = if enemy.max_health - enemy.health <= 5 {
        2 // Only Attack or Defend
    } else {
        3 // Can Heal
    };
    
    // Enemy is more likely to defend if health is decent but not full
    let min_action: i32 = if enemy.max_health < enemy.health * 3 {
        2
    } else {
        1
    };
    
    let action: i32 = rand::thread_rng().gen_range(min_action..=max_action);
    match action {
        1 => BattleAction::Attack,
        2 => BattleAction::Defend,
        3 => BattleAction::Heal,
        _ => unreachable!(),
    }
}

/// Calculates the damage dealt by an attacker to a defender, accounting for defense and blocking.
pub fn calculate_damage(attacker: &Character, defender: &Character) -> i32 {
    let mut dmg = attacker.attack_roll() - defender.defense;
    if defender.is_blocking {
        dmg /= 2;
    }
    if dmg < 0 {
        dmg = 0;
    }
    dmg
}

/// Executes a battle action, modifying the characters' states and displaying the result.
pub fn apply_action(attacker: &mut Character, defender: &mut Character, action: BattleAction) {
    match action {
        BattleAction::Attack => {
            let damage = calculate_damage(attacker, defender);
            defender.take_damage(damage);
            print_box(attacker, defender, damage, BoxType::Attack);
        }
        BattleAction::Defend => {
            attacker.is_blocking = true;
            print_box(attacker, defender, 0, BoxType::Defend);
        }
        BattleAction::Heal => {
            attacker.heal(5);
            print_box(attacker, defender, 5, BoxType::Heal);
        }
        BattleAction::Exit => {
            clear_screen();
            print_ascii_banner(5);
            exit(0)
        }
    }
}

/// Resets the blocking state for both characters at the end of a round.
pub fn reset_blocking(player: &mut Character, enemy: &mut Character) {
    player.is_blocking = false;
    enemy.is_blocking = false;
}

/// Handles the logic for when the player wins a battle (restores health, shows victory screen).
pub fn win_handler(player: &mut Character, enemy: &Character) -> bool {
    let amount_restored = player.max_health - player.health;
    player.health = player.max_health;
    println!("{} defeated {}", player.name, enemy.name);
    println!("Restored {} HP to {}", amount_restored, player.name);
    let menu_action = get_play_again_action();
    print_ascii_banner(6);
    println!();
    match menu_action {
        MenuAction::Exit => {
            print_ascii_banner(5);
            exit(0)
        }
        MenuAction::PlayAgain => {
            print_ascii_banner(7);
            true
        }
    }
}

/// Handles the logic for when the player loses a battle (shows defeat screen, prompts for restart).
pub fn lose_handler(player: &mut Character, enemy: &Character) -> bool {
    println!("{} lose to {}", player.name, enemy.name);
    let menu_action = get_play_again_action();
    match menu_action {
        MenuAction::Exit => {
            print_ascii_banner(5);
            exit(0)
        }
        MenuAction::PlayAgain => {
            print_ascii_banner(7);
            true
        }
    }
}
