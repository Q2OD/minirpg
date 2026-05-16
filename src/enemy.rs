use rand::Rng;
use crate::character::Character;

/// Represents the different species/types of enemies that can be encountered.
pub enum EnemyKind {
    Slime,
    Goblin,
    Orc,
    Undead,
}

/// Generates a random enemy encounter based on the player's current level.
pub fn encounter_enemy(player: &Character) -> Character {
    let random_kind = rand::thread_rng().gen_range(0..=3);
    // Enemies can be up to 3 levels higher than the player
    let maxlevel = player.level + 3;
    let kind = match random_kind {
        0 => EnemyKind::Slime,
        1 => EnemyKind::Goblin,
        2 => EnemyKind::Orc,
        3 => EnemyKind::Undead,
        _ => unreachable!(),
    };

    Character::new_enemy(kind, maxlevel)
}
