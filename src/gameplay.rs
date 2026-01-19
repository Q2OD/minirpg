use bevy::prelude::*;

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum AppState {
    #[default]
    Overworld,
    Battle,
    Menu,
}

#[derive(Resource, Default)]
pub struct RpgEngine {
    pub ticks: u64,
}
