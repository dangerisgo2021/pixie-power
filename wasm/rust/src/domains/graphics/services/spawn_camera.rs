use bevy::prelude::*;
use bevy::render::camera::ScalingMode;

pub fn spawn_camera(mut commands: Commands) {
    let projection = OrthographicProjection {
        // scaling_mode: ScalingMode::FixedHorizontal(1000.),
        near: -0.1,
        ..Default::default()
    };
    
    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(0., 0., 0.),
        projection,
        ..default()
    });
}
