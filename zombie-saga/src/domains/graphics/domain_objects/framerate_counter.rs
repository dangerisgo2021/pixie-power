use bevy::prelude::*;

/// Marker to find the container entity so we can show/hide the FPS counter
#[derive(Component)]
pub struct FramerateCounter;

#[derive(Component)]
pub struct FramerateText;

