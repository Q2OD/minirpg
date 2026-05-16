use rand::Rng;
use crate::utils::{input, roll_stat};
use crate::ui::{clear_screen, turn_separator};

/// Events that can trigger changes to a character's XP or level.
pub enum XpEvents {
    Gain(i32),
    Lose(i32),
    LevelUp,
}

/// The core entity representing both the player and enemies in the game.
pub struct Character {
    pub name: String,
    pub max_health: i32,
    pub health: i32,
    pub max_attack: i32,
    pub defense: i32,
    pub level: i32,
    pub xp: i32,
    pub xp_to_next: i32,
    pub is_blocking: bool,
}

impl Character {
    /// Interactive constructor that prompts the user to name their character and rolls starting stats.
    pub fn new_character() -> Character {
        println!("Please enter a character name");
        let player_name = input();
        clear_screen();
        
        // Randomly determine starting attributes
        let max_health = roll_stat("Health", 25, 50);
        let attack = roll_stat("Max Attack", 5, 8);
        let defense = roll_stat("Defense", 3, 7);
        
        // Initialize base level stats
        let level = 1;
        let xp = 0;
        let xp_to_next = Self::xp_needed_for_level(level);
        let health = max_health;
        let blocking = false;

        let player = Character {
            name: player_name,
            health,
            max_health,
            max_attack: attack,
            level,
            xp,
            xp_to_next,
            defense,
            is_blocking: blocking,
        };
        
        println!("Your character has been created.");
        player.print_stats();
        turn_separator();
        player
    }

    /// Factory method to create a new enemy with stats scaled by its level.
    pub fn new_enemy(kind: crate::enemy::EnemyKind, maxlevel: i32) -> Character {
        let mut minlevel = maxlevel - 5;
        if minlevel <= 0 {
            minlevel = 1
        }
        let level = rand::thread_rng().gen_range(minlevel..=maxlevel);
        
        // Scaling factor based on level (5% per level)
        let factor = 1.0 + (level as f32 * 0.05);
        
        match kind {
            crate::enemy::EnemyKind::Slime => {
                let base_health = 10;
                let base_attack = 5;
                let base_defense = 3;
                Character {
                    name: "Slime".to_string(),
                    health: (base_health as f32 * factor) as i32,
                    max_health: (base_health as f32 * factor) as i32,
                    max_attack: (base_attack as f32 * factor) as i32,
                    defense: (base_defense as f32 * factor) as i32,
                    xp_to_next: 0,
                    xp: 0,
                    level,
                    is_blocking: false,
                }
            }
            crate::enemy::EnemyKind::Undead => {
                let base_health = 20;
                let base_attack = 5;
                let base_defense = 2;
                Character {
                    name: "Undead".to_string(),
                    health: (base_health as f32 * factor) as i32,
                    max_health: (base_health as f32 * factor) as i32,
                    max_attack: (base_attack as f32 * factor) as i32,
                    defense: (base_defense as f32 * factor) as i32,
                    xp_to_next: 0,
                    xp: 0,
                    level,
                    is_blocking: false,
                }
            }
            crate::enemy::EnemyKind::Goblin => {
                let base_health = 15;
                let base_attack = 4;
                let base_defense = 2;
                Character {
                    name: "Goblin".to_string(),
                    health: (base_health as f32 * factor) as i32,
                    max_health: (base_health as f32 * factor) as i32,
                    max_attack: (base_attack as f32 * factor) as i32,
                    defense: (base_defense as f32 * factor) as i32,
                    xp_to_next: 0,
                    xp: 0,
                    level,
                    is_blocking: false,
                }
            }
            crate::enemy::EnemyKind::Orc => {
                let base_health = 30;
                let base_attack = 7;
                let base_defense = 3;
                Character {
                    name: "Orc".to_string(),
                    health: (base_health as f32 * factor) as i32,
                    max_health: (base_health as f32 * factor) as i32,
                    max_attack: (base_attack as f32 * factor) as i32,
                    defense: (base_defense as f32 * factor) as i32,
                    xp_to_next: 0,
                    xp: 0,
                    level,
                    is_blocking: false,
                }
            }
        }
    }

    /// Calculates the amount of XP required to reach the next level.
    pub fn xp_needed_for_level(level: i32) -> i32 {
        10 * (level * level)
    }

    /// Calculates how much XP the player gains for defeating this enemy.
    pub fn xp_gain(enemy: &mut Character) -> XpEvents {
        let reward = 2 * enemy.level;
        XpEvents::Gain(reward)
    }

    /// Calculates how much XP the player loses for being defeated by this enemy.
    pub fn xp_lose(enemy: &mut Character) -> XpEvents {
        let penalty = 3 * enemy.level;
        XpEvents::Lose(penalty)
    }

    /// Updates the character's XP based on an event and handles level-ups.
    pub fn apply_xp(&mut self, event: XpEvents) {
        match event {
            XpEvents::Gain(amount) => self.xp += amount,
            XpEvents::Lose(amount) => {
                if self.xp < amount {
                    self.xp = 0
                } else {
                    self.xp -= amount
                }
            }
            XpEvents::LevelUp => {
                while self.xp >= self.xp_to_next {
                    self.level += 1;
                    self.xp -= self.xp_to_next
                }
            }
        }
        
        // Recursive level-up check
        while self.xp >= self.xp_to_next {
            self.level += 1;
            self.xp -= self.xp_to_next
        }
    }

    /// Returns true if the character's current health is greater than 0.
    pub fn is_alive(&self) -> bool {
        self.health > 0
    }

    /// Rolls for damage using the character's max_attack as the upper bound.
    pub fn attack_roll(&self) -> i32 {
        let max = if self.max_attack < 1 {
            1
        } else {
            self.max_attack
        };

        rand::thread_rng().gen_range(1..=max)
    }

    /// Reduces the character's health by the specified amount, preventing it from dropping below 0.
    pub fn take_damage(&mut self, amount: i32) {
        let damage: i32 = self.health - amount;
        if damage < 0 {
            self.health = 0;
        } else {
            self.health = damage;
        }
    }

    /// Restores health up to the character's max_health.
    pub fn heal(&mut self, amount: i32) {
        let health: i32 = self.health + amount;
        if health > self.max_health {
            self.health = self.max_health;
        } else {
            self.health = health;
        }
    }

    /// Displays the character's primary stats to the console.
    pub fn print_stats(&self) {
        println!("Name: {}", self.name);
        println!("Level: {}", self.level);
        println!("Health: {}/{}", self.health, self.max_health);
        println!("Max Attack: {}", self.max_attack);
        println!("Defense: {}", self.defense);
    }
}
