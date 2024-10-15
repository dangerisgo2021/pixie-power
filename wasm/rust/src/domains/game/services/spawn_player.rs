use crate::domains::game::domain_objects::piece::Piece;
use crate::domains::game::domain_objects::player::Player;
use crate::domains::game::value_objects::position::Position;
use crate::domains::graphics::resources::sprite_atlas::SpriteSheetAtlas;
use bevy::prelude::*;
use crate::domains::game::domain_objects::movable::Movable;
use crate::domains::game::value_objects::direction::Direction;

pub fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    atlas_layout: Res<SpriteSheetAtlas>,
) {
    commands.spawn((
        Player,
        Movable {
            position: Position {x: 0, y: 0},
            direction: Direction::None,
            movement_speed: 0,
            // direction: Direction::Right,
            // movement_speed: 1,
        },
        Piece {
            position: Position { x: 0, y: 0 },
            sprite: SpriteBundle {
                texture: asset_server.load("images\\fairy-spritesheet.png"),
                ..default()
            },
            atlas: TextureAtlas {
                layout: atlas_layout.handle.clone(),
                index: 2,
            },
        },
    ));
}
