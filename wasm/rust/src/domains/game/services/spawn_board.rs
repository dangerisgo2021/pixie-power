use crate::domains::game::domain_objects::grid_square::GridSquare;
use crate::domains::game::value_objects::position::Position;
use bevy::prelude::*;

pub fn spawn_board(mut commands: Commands) {
    let width = 11;
    let height = 11;
    let square_size = 16.0;
    for row in 0..height {
        for col in 0..width {
            let index = row * height + col;
            let square_position =
                Vec3::new(col as f32 * (square_size), row as f32 * (square_size), -5.0);

            let square_color = if index % 2 == 0 {
                Color::linear_rgb(1.0, 0.7, 0.4)
            } else {
                Color::linear_rgb(0.4, 1.0, 0.6)
            };

            commands.spawn((
                GridSquare {
                    position: Position::new(col, row),
                },
                SpriteBundle {
                    transform: Transform {
                        translation: square_position,
                        scale: Vec3::new(square_size, square_size, 1.0),
                        ..default()
                    },
                    sprite: Sprite {
                        color: square_color,
                        ..default()
                    },
                    ..default()
                },
            ));
        }
    }
}
