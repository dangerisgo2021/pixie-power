use bevy::prelude::*;

#[derive(Resource)]
pub struct SnakeGame {
    pub width: i32,
    pub height: i32,
    pub square_size: f32,
    pub grid_level: f32,
    pub square_color_primary: Color,
    pub square_color_secondary: Color,
    pub scale: f32,
}