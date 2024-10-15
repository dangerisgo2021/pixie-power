use bevy::app::{App, Plugin, Startup};
use bevy::DefaultPlugins;
use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::prelude::{ImagePlugin, Msaa, PluginGroup};
use crate::domains::graphics::resources::sprite_atlas::SpriteSheetAtlas;
use crate::domains::graphics::services::spawn_camera::spawn_camera;
use crate::domains::graphics::plugins::framerate_counter_plugin::FramerateCounterPlugin;

pub struct GraphicsPlugin;

impl Plugin for GraphicsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SpriteSheetAtlas>();
        app.insert_resource(Msaa::Off);
        app.add_systems(Startup, spawn_camera);
        app.add_plugins(FrameTimeDiagnosticsPlugin);
        app.add_plugins(FramerateCounterPlugin);
        // app.add_systems(Update, adjust_transforms);
    }
}
