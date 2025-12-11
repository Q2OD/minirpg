// ▗▖ ▗▖▗▞▀▚▖█ ▗▞▀▘ ▄▄▄  ▄▄▄▄  ▗▞▀▚▖       ■   ▄▄▄      ▗▖  ▗▖▄ ▄▄▄▄  ▄     ▗▄▄▖ ▗▄▄▖  ▗▄▄▖
// ▐▌ ▐▌▐▛▀▀▘█ ▝▚▄▖█   █ █ █ █ ▐▛▀▀▘    ▗▄▟▙▄▖█   █     ▐▛▚▞▜▌▄ █   █ ▄     ▐▌ ▐▌▐▌ ▐▌▐▌   
// ▐▌ ▐▌▝▚▄▄▖█     ▀▄▄▄▀ █   █ ▝▚▄▄▖      ▐▌  ▀▄▄▄▀     ▐▌  ▐▌█ █   █ █     ▐▛▀▚▖▐▛▀▘ ▐▌▝▜▌
// ▐▙█▟▌     █                            ▐▌            ▐▌  ▐▌█       █     ▐▌ ▐▌▐▌   ▝▚▄▞▘
//                                        ▐▌                                               
// ▗▄▄▖ ▄   ▄      ▗▄▄▖▗▞▀▜▌█ ▗▞▀▚▖▗▖       ▗▖  ▗▖▗▞▀▜▌█  ▐▌
// ▐▌ ▐▌█   █     ▐▌   ▝▚▄▟▌█ ▐▛▀▀▘▐▌       ▐▛▚▞▜▌▝▚▄▟▌▀▄▄▞▘
// ▐▛▀▚▖ ▀▀▀█     ▐▌        █ ▝▚▄▄▖▐▛▀▚▖    ▐▌  ▐▌          
// ▐▙▄▞▘▄   █     ▝▚▄▄▖     █      ▐▙▄▞▘    ▐▌  ▐▌          
//       ▀▀▀                                                                                                                                                                                                                          
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
            xp: 0,
            xp_to_next: 10,
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
    fn xp_needed_for_level(level: i32) -> i32 {
        let next_level_xp = 10 * level * level;
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
                    self.level = self.level + 1
                }
            }
        }
        while self.xp >= self.xp_to_next {

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
enum Action {
    Attack,
    Defend,
    Heal,
    Exit,
}
enum XpEvents {
    Gain(i32),
    Lose(i32),
    LevelUp,
}
// Main Game Function
fn main() {
    println!(
    "                                                                                                                                                                                                                                                                                                                                       
                                                                                                                                                                                                                                                                                                                                       
▗▖ ▗▖▗▞▀▚▖█ ▗▞▀▘ ▄▄▄  ▄▄▄▄  ▗▞▀▚▖       ■   ▄▄▄      ▗▖  ▗▖▄ ▄▄▄▄  ▄     ▗▄▄▖ ▗▄▄▖  ▗▄▄▖
▐▌ ▐▌▐▛▀▀▘█ ▝▚▄▖█   █ █ █ █ ▐▛▀▀▘    ▗▄▟▙▄▖█   █     ▐▛▚▞▜▌▄ █   █ ▄     ▐▌ ▐▌▐▌ ▐▌▐▌   
▐▌ ▐▌▝▚▄▄▖█     ▀▄▄▄▀ █   █ ▝▚▄▄▖      ▐▌  ▀▄▄▄▀     ▐▌  ▐▌█ █   █ █     ▐▛▀▚▖▐▛▀▘ ▐▌▝▜▌
▐▙█▟▌     █                            ▐▌            ▐▌  ▐▌█       █     ▐▌ ▐▌▐▌   ▝▚▄▞▘
                                       ▐▌                                               
                                                                                        
                                                                                        ");
    let mut player = Character::new_character();
    game_loop(&mut player);
}
fn game_loop(player: &mut Character) {
    loop {
        let mut enemy = encounter_enemy(&player);
        let game = battle(player, &mut enemy);
        if game {
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
            println!("You were defeated...");
            return false;
        }
        if !enemy.is_alive() {
            println!("You win!");
            return true;
        }
        print_status(player, enemy);
        let player_action = get_player_action();
        let enemy_action = get_enemy_action(enemy);
        apply_action(player, enemy, player_action);
        if !enemy.is_alive() {
            println!("You win!");
            return true;
        }
        apply_action(enemy, player, enemy_action);
        if !player.is_alive() {
            println!("You were defeated...");
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
    let min = 1;
    let value = rand::thread_rng().gen_range(min..=max);
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
fn get_player_action () -> Action {
    println!("Choose your action: \n 1) Attack \n 2) Defend \n 3) Heal \n 4) Exit");
    let choice = input();
    match choice.as_str () {
        "1" => Action::Attack,
        "2" => Action::Defend,
        "3" => Action::Heal,
        "4" => Action::Exit,
        _ => {
            println!("Invalid Choice, try again.");
            get_player_action()
        }
    }
}
fn get_enemy_action (_enemy: &Character) -> Action {
    let action: i32 = rand::thread_rng().gen_range(1..=3);
    match action {
        1 => Action::Attack,
        2 => Action::Defend,
        3 => Action::Heal,
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
fn apply_action(attacker: &mut Character, defender: &mut Character, action: Action) {
    match action {
        Action::Attack => {
            let damage = calculate_damage(attacker, defender);
            defender.take_damage(damage);
            println!("{}, Attacks {}, takes {} HP from {}", attacker.name, defender.name, damage, defender.name)
        }
        Action::Defend => {
            attacker.is_blocking = true;
            println!("{} is Blocking", attacker.name)
        }
        Action::Heal => {
            attacker.heal(5);
            println!("{} Heals 5 HP leaving them at {} HP", attacker.name, attacker.health);
        }
        Action::Exit => {
            println!("Thank you for playing!");
            exit(1)
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