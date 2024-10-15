use crate::domains::game::plugins::game_plugin::GamePlugin;
use crate::domains::graphics::plugins::graphics_plugin::GraphicsPlugin;
use crate::domains::menus::plugins::menu_plugin::MenuPlugin;

use bevy::{prelude::*};

pub fn start_app() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins((GraphicsPlugin, GamePlugin, MenuPlugin))
        .run();
}
