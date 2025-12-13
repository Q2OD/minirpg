# MINI RPG

Mini RPG is a turn-based, text-only adventure that runs entirely in your terminal. It started as a school assignment and is now evolving into a tiny playground for experimenting with Rust, game loops, and combat mechanics. Each play session spins up a new hero, throws increasingly tough enemies at you, and lets you practice balancing offense, defense, and healing in fast rounds.

## Highlights
- **Instant character creation** – enter a name and receive randomized core stats (health, attack, defense) so every run feels a little different.
- **Turn-based battles** – trade blows with Slimes, Goblins, Orcs, and Undead; the AI makes situational decisions such as blocking when low on health.
- **Simple action economy** – each turn you can Attack, Defend (halves incoming damage), Heal, or Exit the run.
- **Retro presentation** – ASCII art banners and straightforward status readouts keep the experience lightweight and terminal-friendly.
- **Extensible Rust codebase** – structs such as `Character` and helpers like `apply_action` isolate logic so new mechanics or enemy types can be added incrementally.

## Requirements
- Rust toolchain that supports the 2024 edition (Rust 1.76+ recommended).
- Cargo (installed alongside Rust).

## Getting Started
```bash
git clone https://github.com/Q2OD/minirpg
cd rpg
cargo run
```
1. When the ASCII banner appears, enter a hero name.
2. Watch the console log your randomly rolled stats.
3. Continue following the prompts during combat.

The binary is small, so `cargo run` is enough for most workflows. Use `cargo build --release` if you want a faster executable for repeated playtests.

## Gameplay Loop
1. **Encounter roll** – after each victory the game heals you, then spawns a random enemy between your level and +3 (never below level 1).
2. **Command phase** – pick between Attack, Defend, Heal, or Exit via the numeric menu.
3. **Resolution** – the player acts first, the enemy responds, and both blocking flags reset for the next round.
4. **Win/Loss** – defeating an enemy shows a victory banner; hitting 0 HP ends the run.

### Actions
- `Attack`: rolls up to your `max_attack`, subtracts the enemy’s defense, then applies damage (minimum 0). Blocking on the defender halves the result.
- `Defend`: toggles a blocking flag that halves damage until the round ends.
- `Heal`: restores 5 HP without exceeding `max_health`.
- `Exit`: quits gracefully if you want to stop the run early.

### Experience Hooks
`Character` already includes XP bookkeeping plus helper enums (`XpEvents`), but the current loop only heals you between fights. This makes it easy to add leveling later by adjusting `apply_xp` inside `battle` or `game_loop`. The README keeps this callout so contributors know where to start.

## Project Layout
```
rpg/
├── Cargo.toml
├── src/
│   └── main.rs   # all gameplay logic, ASCII banners, helpers
└── README.MD
```

The game intentionally stays in a single binary crate for now. Refactoring ideas: move combat logic into modules, split enemy definitions, or turn `Attacks` into real abilities.

## Contributing / Next Ideas
1. Wire XP events into battle outcomes so winning/losing affects leveling.
2. Flesh out the `Attacks` struct to allow multi-use skills or consumables.
3. Add tests (e.g., damage calculation, blocking math) to guard future refactors.
4. Expand the world with story prompts, shops, or equipment drops.

Feel free to open issues or PRs with any experiments—this project is intentionally open-ended and perfect for learning Rust while building a tiny RPG.

---

_Changelog_
- Fri Dec 12 – GitHub repo created.
