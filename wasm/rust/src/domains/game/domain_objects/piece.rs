use bevy::prelude::*;
use crate::domains::game::value_objects::position::Position;

#[derive(Bundle)]
pub struct Piece {
    pub position: Position,
    pub sprite: SpriteBundle,
    pub atlas: TextureAtlas,
}