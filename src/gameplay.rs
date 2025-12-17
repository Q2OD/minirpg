use bevy::prelude::*;

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum AppState {
    #[default]
    Overworld,
    Battle,
    Menu,
}

/// Replace this with your real RPG engine/structs.
/// The point is: your existing logic becomes a Resource.
#[derive(Resource, Default)]
pub struct RpgEngine {
    pub ticks: u64,
}
