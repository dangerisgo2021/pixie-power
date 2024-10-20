use bevy::prelude::*;
use bevy::render::camera::ScalingMode;

pub fn spawn_camera(mut commands: Commands) {
    let projection = OrthographicProjection {
        scaling_mode: ScalingMode::WindowSize(2.),
        near: -0.1,
        ..Default::default()
    };
    
    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(70.0, 80.0, 0.0),
        projection,
        ..default()
    });
}
