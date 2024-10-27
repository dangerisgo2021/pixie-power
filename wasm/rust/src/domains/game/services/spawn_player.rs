
use crate::domains::game::domain_objects::sprite_with_atlas::SpriteWithAtlas;
use crate::domains::game::domain_objects::player::{Player, SnakeNode};
use crate::domains::game::value_objects::direction::Direction;
use crate::domains::game::value_objects::position::Position;
use crate::domains::graphics::resources::sprite_atlas::SpriteSheetAtlas;
use bevy::prelude::*;

pub fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    atlas_layout: Res<SpriteSheetAtlas>,
) {
    println!("spawn_tail");
    //add part to tail
    // commands.spawn((
    //     SnakeNode {
    //         position: Position { x: 1, y: 4 },
    //         index: 0,
    //     },
    //     Collidable,
    //     Piece {
    //         sprite: SpriteBundle {
    //             texture: asset_server.load("images\\fairy-spritesheet.png"),
    //             transform: Transform {
    //                 translation: Vec3::new(-16.0, 0.0, -1.),
    //                 ..default()
    //             },
    //             ..default()
    //         },
    //         atlas: TextureAtlas {
    //             layout: atlas_layout.handle.clone(),
    //             index: 3,
    //         },
    //     },
    // ));
    // 
    // println!("spawn_tail");
    // //add part to tail
    // commands.spawn((
    //     SnakeNode {
    //         position: Position { x: 0, y: 4 },
    //         index: 1,
    //     },
    //     Collidable,
    //     Piece {
    //         sprite: SpriteBundle {
    //             texture: asset_server.load("images\\fairy-spritesheet.png"),
    //             transform: Transform {
    //                 translation: Vec3::new(-32.0, 0.0, -1.),
    //                 ..default()
    //             },
    //             ..default()
    //         },
    //         atlas: TextureAtlas {
    //             layout: atlas_layout.handle.clone(),
    //             index: 3,
    //         },
    //     },
    // ));
    // spawn player
    commands.spawn((
        Player {
            tail_length: 2,
            position: Position { x: 2, y: 4 },
            direction: Direction::None,
            can_change_direction: true,
        },
        SpriteWithAtlas {
            sprite: SpriteBundle {
                texture: asset_server.load("images\\fairy-spritesheet.png"),
                transform: Transform {
                    translation: Vec3::new(32.0, 64., 0.),
                    ..default()
                },
                ..default()
            },
            atlas: TextureAtlas {
                layout: atlas_layout.handle.clone(),
                index: 2,
            },
        },
    ));
}
