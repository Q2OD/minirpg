# Mini RPG Development Roadmap

1. **Refactor & Modularize Codebase**
   - Split `src/main.rs` into smaller, focused modules:
     - `src/character.rs`: `Character` struct, level/XP logic, and character methods.
     - `src/enemy.rs`: `EnemyKind` enum and enemy generation factory.
     - `src/battle.rs`: Combat loop, damage calculation, and battle action handling.
     - `src/ui.rs`: ASCII banners, screen clearing, and status display.
     - `src/utils.rs`: Input handling and random number utility functions.
   - Update `main.rs` to act as the entry point and orchestrator.
   - Ensure proper use of `pub` visibility and module declarations.

2. **Add New Attacks**
   - Implement the `Attacks` struct.
   - Integrate attack variety into the battle system.

3. **Expand XP System**
   - Finalize XP gain/loss mechanics.
   - Implement persistent level-up bonuses.
