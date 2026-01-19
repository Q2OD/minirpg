use bevy::prelude::*;
use crate::gameplay::{AppState, RpgEngine};
pub struct GamePlugin;
impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<AppState>()
            .init_resource::<RpgEngine>()
            .add_systems(Startup, setup_ui)
            .add_systems(Update, tick_engine);
    }
}
fn setup_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);

    commands.spawn((
        Text::new("Basic UI Text, Not a game"),
        TextFont {
            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
            font_size: 48.0,
            ..default()
        },
        TextLayout::new_with_justify(Justify::Left),
        Node {
            position_type: PositionType::Absolute,
            top: px(20),
            left: px(20),
            ..default()
        },
    ));
}
fn tick_engine(mut engine: ResMut<RpgEngine>) {
    engine.ticks += 1;
}
