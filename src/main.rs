// ▗▖ ▗▖▗▄▄▄▖▗▖    ▗▄▄▖ ▗▄▖ ▗▖  ▗▖▗▄▄▄▖    ▗▄▄▄▖▗▄▖     ▗▖  ▗▖▗▄▄▄▖▗▖  ▗▖▗▄▄▄▖    ▗▄▄▖ ▗▄▄▖  ▗▄▄▖
// ▐▌ ▐▌▐▌   ▐▌   ▐▌   ▐▌ ▐▌▐▛▚▞▜▌▐▌         █ ▐▌ ▐▌    ▐▛▚▞▜▌  █  ▐▛▚▖▐▌  █      ▐▌ ▐▌▐▌ ▐▌▐▌   
// ▐▌ ▐▌▐▛▀▀▘▐▌   ▐▌   ▐▌ ▐▌▐▌  ▐▌▐▛▀▀▘      █ ▐▌ ▐▌    ▐▌  ▐▌  █  ▐▌ ▝▜▌  █      ▐▛▀▚▖▐▛▀▘ ▐▌▝▜▌
// ▐▙█▟▌▐▙▄▄▖▐▙▄▄▖▝▚▄▄▖▝▚▄▞▘▐▌  ▐▌▐▙▄▄▖      █ ▝▚▄▞▘    ▐▌  ▐▌▗▄█▄▖▐▌  ▐▌▗▄█▄▖    ▐▌ ▐▌▐▌   ▝▚▄▞▘                                                                                              
// ▗▄▄▖▗▖  ▗▖     ▗▄▄▖ ▗▄▖ ▗▖   ▗▄▄▄▖▗▄▄▖     ▗▖  ▗▖ ▗▄▖ ▗▖ ▗▖
// ▐▌ ▐▌▝▚▞▘     ▐▌   ▐▌ ▐▌▐▌   ▐▌   ▐▌ ▐▌    ▐▛▚▞▜▌▐▌ ▐▌▐▌ ▐▌
// ▐▛▀▚▖ ▐▌      ▐▌   ▐▛▀▜▌▐▌   ▐▛▀▀▘▐▛▀▚▖    ▐▌  ▐▌▐▛▀▜▌▐▌ ▐▌
// ▐▙▄▞▘ ▐▌      ▝▚▄▄▖▐▌ ▐▌▐▙▄▄▖▐▙▄▄▖▐▙▄▞▘    ▐▌  ▐▌▐▌ ▐▌▝▚▄▞▘                                                                                                                                                                                                                                        
#![allow(dead_code)]
// STD Libs
use std::io;
use std::process::exit;
// Crates
use rand::{Rng};
// Structs

struct Attacks {
    name: String, 
    damage: i32,
    factor: f32,
    max_damage: i32,
    uses_left: u8,
}
struct Character {
    name: String,
    max_health: i32,
    health: i32,
    max_attack: i32,
    defense: i32,
    level: i32,
    xp: i32,
    xp_to_next: i32,
    is_blocking: bool
}
// Methods
impl Attacks {
}
impl Character {
    fn new_character() -> Character {
        println!("Please enter a character name");
        let player_name = input();
        // Random Stats
        let max_health = roll_stat("Health", 25, 50);
        let attack = roll_stat("Max Attack", 5, 8);
        let defense = roll_stat("Defense", 3, 7);
        // Static Stats
        let level = 1;
        let xp = 0;
        let xp_to_next = Self::xp_needed_for_level(level);
        let health = max_health;
        let blocking = false;

        let player = Character {
            name: player_name,
            health: health,
            max_health: max_health,
            max_attack: attack,
            level: level,
            xp,
            xp_to_next,
            defense: defense,
            is_blocking: blocking
        };
        println!("Your character has been created.");
        player.print_stats();
        return player
    }

    fn new_enemy(kind: EnemyKind, maxlevel: i32) -> Character {
        let mut minlevel = maxlevel - 5;
        if minlevel <= 0 {
            minlevel = 1
        } else {
        }
        let level = rand::thread_rng().gen_range(minlevel..=maxlevel);
        let factor = 1.0 + (level as f32 * 0.05);
        match kind {
            EnemyKind::Slime => {
                let base_health = 10;
                let base_attack = 5;
                let base_defense = 3;

                let health = (base_health as f32 * factor) as i32;
                let attack = (base_attack as f32 * factor) as i32;
                let defense = (base_defense as f32 * factor) as i32;
                Character {
                    name: "Slime".to_string(),
                    health,
                    max_health: health,
                    max_attack: attack,
                    defense,
                    xp_to_next: 0,
                    xp: 0,
                    level: level,
                    is_blocking: false
                }
            }
            EnemyKind::Undead => {
                let base_health = 20;
                let base_attack = 5;
                let base_defense = 2;

                let health = (base_health as f32 * factor) as i32;
                let attack = (base_attack as f32 * factor) as i32;
                let defense = (base_defense as f32 * factor) as i32;
                Character {
                    name: "Undead".to_string(),
                    health,
                    max_health: health,
                    max_attack: attack,
                    defense,
                    xp_to_next: 0,
                    xp: 0,
                    level: level,
                    is_blocking: false
                }
            }
            EnemyKind::Goblin => {
                let base_health = 15;
                let base_attack = 4;
                let base_defense = 2;

                let health = (base_health as f32 * factor) as i32;
                let attack = (base_attack as f32 * factor) as i32;
                let defense = (base_defense as f32 * factor) as i32;
                Character {
                    name: "Goblin".to_string(),
                    health,
                    max_health: health,
                    max_attack: attack,
                    defense,
                    xp_to_next: 0,
                    xp: 0,
                    level: level,
                    is_blocking: false
                }
            }
            EnemyKind::Orc => {
                let base_health = 30;
                let base_attack = 7;
                let base_defense = 3;

                let health = (base_health as f32 * factor) as i32;
                let attack = (base_attack as f32 * factor) as i32;
                let defense = (base_defense as f32 * factor) as i32;
                Character {
                    name: "Orc".to_string(),
                    health,
                    max_health: health,
                    max_attack: attack,
                    defense,
                    xp_to_next: 0,
                    xp: 0,
                    level: level,
                    is_blocking: false
                }
            }
        }
    }
    // XP Functions
    fn xp_needed_for_level(level: i32) -> i32 {
        let next_level_xp = 10 * (level * level);
        return next_level_xp
    }
    fn xp_gain(enemy: &mut Character) -> XpEvents {
        let reward = 2 * enemy.level;
        return XpEvents::Gain(reward)
    }
    fn xp_lose(enemy: &mut Character) -> XpEvents {
        let penalty = 3 * enemy.level;
        return XpEvents::Lose(penalty)
    }
    fn apply_xp(&mut self, event: XpEvents) {
        match event {
            XpEvents::Gain(amount) => {
                self.xp = self.xp + amount
            }
            XpEvents::Lose(amount) => {
                if self.xp < amount {
                    self.xp = 0
                } else {
                    self.xp = self.xp - amount
                }
            }
            XpEvents::LevelUp => {
                while self.xp >= self.xp_to_next {
                    self.level = self.level + 1;
                    self.xp = self.xp - self.xp_to_next
                }
            } 
        }
        // while self.xp < 0 {
        //     self.level = self.level - 1 
        // }
        while self.xp >= self.xp_to_next {
                    self.level = self.level + 1;
                    self.xp = self.xp - self.xp_to_next
        }
    }
    fn is_alive(&self) -> bool {
        self.health > 0
    }
    fn attack_roll(&self) -> i32 {
        let max = if self.max_attack < 1 { 1 } else { self.max_attack };
        let roll = rand::thread_rng().gen_range(1..=max);
        return roll
    }
    fn take_damage(&mut self, amount: i32) {
        let damage: i32 = self.health - amount;
        if damage < 0 {
            self.health = 0;
        } else {
            self.health = damage;
        }
    }
    fn heal(&mut self, amount: i32) {
        let health: i32 = self.health + amount;
        if health > self.max_health {
            self.health = self.max_health;
        } else {
            self.health = health;
        }
    }
    fn print_stats(&self) {
        println!("Name: {}", self.name);
        println!("Level: {}", self.level);
        println!("Health: {}/{}", self.health, self.max_health);
        println!("Max Attack: {}", self.max_attack);
        println!("Defense: {}", self.defense);
    }
}
// Enums
enum EnemyKind {
    Slime,
    Goblin,
    Orc,
    Undead
}
enum BattleAction {
    Attack,
    Defend,
    Heal,
    Exit,
}
enum MenuAction {
    Exit,
    Playagain
}
enum XpEvents {
    Gain(i32),
    Lose(i32),
    LevelUp,
}
// Main Game Function
fn main() {
    print_ascii_banner(1);
    println!("Alpha V1.0.1");
    let mut player = Character::new_character();
    game_loop(&mut player);
}
fn game_loop(player: &mut Character) {
    loop {
        let mut enemy = encounter_enemy(&player);
        let game = battle(player, &mut enemy);
        if game {
            player.health = player.max_health;
            let menu_action = get_play_again_action();
            match menu_action { 
                MenuAction::Exit => {
                    print_ascii_banner(5);
                    exit(0)
                }
                MenuAction::Playagain => {
                    print_ascii_banner(6);
                    continue;
                }
            }
            // Xp Events
        } else {
            println!("You suck get better... bye.");
            exit(0);
        }
    }
}
fn encounter_enemy(player: &Character) -> Character {
    let random_kind = rand::thread_rng().gen_range(0..=3);
    let maxlevel = player.level + 3;
    let kind = match random_kind {
        0 => EnemyKind::Slime,
        1 => EnemyKind::Goblin,
        2 => EnemyKind::Orc,
        3 => EnemyKind::Undead,
        _ => unreachable!(),
    };
    let enemy = Character::new_enemy(kind, maxlevel);
    println!("A level {} {} appears!", enemy.level, enemy.name);
    return enemy
}
fn battle(player: &mut Character, enemy: &mut Character) -> bool {
    loop {
        if !player.is_alive() {
            print_ascii_banner(3);
            return false;
        }
        if !enemy.is_alive() {
            print_ascii_banner(2);
            return true;
        }
        print_status(player, enemy);
        let player_action = get_player_action();
        let enemy_action = get_enemy_action(enemy);
        apply_action(player, enemy, player_action);
        if !enemy.is_alive() {
            print_ascii_banner(2);
            return true;
        }
        apply_action(enemy, player, enemy_action);
        if !player.is_alive() {
            print_ascii_banner(3);
            return false;
        }
        reset_blocking(player, enemy);
    }
}
// HELPERS
fn roll_stat(stat_name: &str, min: i32, max: i32) -> i32 {
    let value = rand::thread_rng().gen_range(min..=max);
    println!(
        "Rolling {} ({}-{})... you got {}!",
        stat_name, min, max, value
    );
    return value
}
fn roll_level_stat(player:&Character, stat_name: &str, min: i32, max: i32) -> i32 {
    let factor = 1.0 + (player.level as f32 * 0.05);
    let min_factored = (min as f32 * factor) as i32;
    let value = rand::thread_rng().gen_range(min_factored..=max);
    println!(
        "Rolling {} ({}-{})... you got {}!",
        stat_name, min, max, value
    );
    return value
}
fn input() -> String {
    let mut line = String::new();
    io::stdin()
        .read_line(&mut line)
        .expect("Failed to read line");
    line.trim().to_string()
}
fn get_player_action () -> BattleAction {
    println!("Choose your action: \n 1) Attack \n 2) Defend \n 3) Heal \n 4) Exit");
    let choice = input();
    match choice.as_str () {
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
fn get_play_again_action () -> MenuAction {
    println!("Play Again? \n y = continue \n n = exit gracefully \n");
    let action = input();
    match action.as_str () {
        "y" => MenuAction::Playagain,
        "n" => MenuAction::Exit,
        _ => {
            println!("Invalid Choice, try again.");
            get_play_again_action()
        }
    }
}
fn get_enemy_action (enemy: &Character) -> BattleAction {
    let max_action: i32;
    if enemy.max_health - enemy.health <= 5 {
        max_action = 2
    } else {
        max_action = 3
    }
    let action: i32 = rand::thread_rng().gen_range(1..=max_action);
    match action {
        1 => BattleAction::Attack,
        2 => BattleAction::Defend,
        3 => BattleAction::Heal,
        _ => unreachable!()
    }
}
fn calculate_damage(attacker: &Character, defender: &Character) -> i32 {
    let mut dmg = attacker.attack_roll() - defender.defense;
    if defender.is_blocking {
        dmg /= 2;
    }
    if dmg < 0 {
        dmg = 0;
    }
    return dmg
}
fn apply_action(attacker: &mut Character, defender: &mut Character, action: BattleAction) {
    match action {
        BattleAction::Attack => {
            let damage = calculate_damage(attacker, defender);
            defender.take_damage(damage);
            println!("{}, Attacks {}, takes {} HP from {}", attacker.name, defender.name, damage, defender.name)
        }
        BattleAction::Defend => {
            attacker.is_blocking = true;
            println!("{} is Blocking", attacker.name)
        }
        BattleAction::Heal => {
            attacker.heal(5);
            println!("{} Heals 5 HP leaving them at {} HP", attacker.name, attacker.health);
        }
        BattleAction::Exit => {
            print_ascii_banner(5);
            exit(0)
        }
    }
}
fn print_status(player: &Character, enemy: &Character) {
    println!("{}: {}/ {} HP", player.name, player.health, player.max_health);
    println!("{}: {}/ {} HP", enemy.name, enemy.health, enemy.max_health);
}
fn reset_blocking(player: &mut Character, enemy: &mut Character) {
    player.is_blocking = false;
    enemy.is_blocking = false;
}
fn print_ascii_banner(ascii: i32) {
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
        _ => unreachable!()
    }
}
