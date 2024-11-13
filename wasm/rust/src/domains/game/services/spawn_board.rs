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

            // when on edge spawn wall instead of grid
            if row == 0 || row == height - 1 || col == 0 || col == width - 1 {
                let position = Position::new(col, row);

                commands.spawn((
                    Wall { position },
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

                commands.spawn((
                    GridSquare {
                        position: Position::new(col, row),
                    },
                    SpriteWithAtlas {
                        sprite: SpriteBundle {
                            texture: asset_server.load("images\\fairy-spritesheet.png"),
                            transform: Transform {
                                translation: Vec3::new(
                                    row as f32 * square_size,
                                    col as f32 * square_size,
                                    snake_game.grid_level,
                                ),
                                ..default()
                            },
                            ..default()
                        },
                        atlas: TextureAtlas {
                            layout: atlas_layout.handle.clone(),
                            index: 5,
                        },
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
                    translation: Vec3::new(
                        6. * snake_game.square_size,
                        4. * snake_game.square_size,
                        -1.,
                    ),
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
