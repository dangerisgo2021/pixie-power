use crate::domains::game::plugins::snake_game_plugin::SnakeGamePlugin;
use bevy::prelude::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(SnakeGamePlugin);
    }
}

