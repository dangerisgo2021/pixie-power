use bevy::prelude::*;
use bevy::render::camera::ScalingMode;

pub fn spawn_camera(mut commands: Commands) {
    let projection = OrthographicProjection {
        scaling_mode: ScalingMode::WindowSize(3.),
        near: -0.1,
        ..Default::default()
    };
    
    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(55., 50.0, 0.0),
        projection,
        ..default()
    });
}
