use crate::domains::game::domain_objects::grid_square::GridSquare;
use crate::domains::game::domain_objects::pickup::Pickup;
use crate::domains::game::domain_objects::snake_game::SnakeGame;
use crate::domains::game::domain_objects::sprite_with_atlas::SpriteWithAtlas;
use crate::domains::game::domain_objects::wall::Wall;
use crate::domains::game::value_objects::position::Position;
use crate::domains::graphics::resources::sprite_atlas::SpriteSheetAtlas;
use bevy::prelude::*;

pub fn spawn_board(
    mut commands: Commands,
    snake_game: Res<SnakeGame>,
    asset_server: Res<AssetServer>,
    atlas_layout: Res<SpriteSheetAtlas>,
) {
    let width = snake_game.width;
    let height = snake_game.height;
    let square_size = snake_game.square_size;

    for row in 0..height {
        for col in 0..width {
            let index = row * height + col;
            let square_position = Vec3::new(
                col as f32 * (square_size),
                row as f32 * (square_size),
                snake_game.grid_level,
            );

            // when on edge spawn wall instead of grid
            if row == 0 || row == height - 1 || col == 0 || col == width - 1 {
                let position = Position::new(col, row);
                println!("spawn wall  row {} col {}, position {:?}", row, col, position);


                commands.spawn((
                    Wall {
                        position,
                    },
                    SpriteWithAtlas {
                        sprite: SpriteBundle {
                            texture: asset_server.load("images\\fairy-spritesheet.png"),
                            transform: Transform {
                                translation: Vec3::new(
                                    row as f32 * square_size,
                                    col as f32 * square_size,
                                    0.,
                                ),
                                ..default()
                            },
                            ..default()
                        },
                        atlas: TextureAtlas {
                            layout: atlas_layout.handle.clone(),
                            index: 1,
                        },
                    },
                ));
            } else {
                let square_color = if index % 2 == 0 {
                    snake_game.square_color_primary
                } else {
                    snake_game.square_color_secondary
                };

                commands.spawn((
                    GridSquare {
                        position: Position::new(col, row),
                    },
                    SpriteBundle {
                        transform: Transform {
                            translation: square_position,
                            scale: Vec3::new(square_size, square_size, snake_game.scale),
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

    commands.spawn((
        Pickup {
            position: Position { x: 6, y: 4 },
        },
        SpriteWithAtlas {
            sprite: SpriteBundle {
                texture: asset_server.load("images\\fairy-spritesheet.png"),
                transform: Transform {
                    translation: Vec3::new(96., 64., -1.),
                    ..default()
                },
                ..default()
            },
            atlas: TextureAtlas {
                layout: atlas_layout.handle.clone(),
                index: 0,
            },
        },
    ));
}
