use crate::domains::graphics::plugins::framerate_counter_plugin::FramerateCounterPlugin;
use crate::domains::graphics::resources::sprite_atlas::SpriteSheetAtlas;
use crate::domains::graphics::services::spawn_camera::spawn_camera;
use bevy::app::{App, Plugin, Startup};
use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::{prelude::*};
use bevy::window::WindowResized;

pub struct GraphicsPlugin;
const LOGICAL_WIDTH: f32 = 350.0;
const LOGICAL_HEIGHT: f32 = 600.0;

impl Plugin for GraphicsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SpriteSheetAtlas>();
        app.insert_resource(Msaa::Off);
        app.add_systems(Startup, spawn_camera);
        app.add_systems(Update, scale_camera);
        app.add_plugins(FrameTimeDiagnosticsPlugin);
        app.add_plugins(FramerateCounterPlugin);
    }
}

fn scale_camera(mut resize_reader: EventReader<WindowResized>, mut camera_query: Query<&mut OrthographicProjection, With<Camera>>,) {
    for resize_event in resize_reader.read() {
        let mut projection = camera_query.single_mut();

        let scale_x = resize_event.width / LOGICAL_WIDTH;
        let scale_y = resize_event.height / LOGICAL_HEIGHT;

        projection.scale = scale_x.min(scale_y); // Maintain aspect ratio
    }

}