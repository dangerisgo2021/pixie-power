use crate::domains::game::plugins::game_plugin::GamePlugin;
use crate::domains::graphics::plugins::graphics_plugin::GraphicsPlugin;
use crate::domains::menus::plugins::menu_plugin::MenuPlugin;

use bevy::prelude::*;
use bevy::window::WindowResolution;

pub fn start_app() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        resizable: true,
                        canvas: Some("#snake-canvas".into()),
                        resolution: WindowResolution::new(400., 600.),
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .add_plugins((GraphicsPlugin, GamePlugin, MenuPlugin))
        .run();
}
