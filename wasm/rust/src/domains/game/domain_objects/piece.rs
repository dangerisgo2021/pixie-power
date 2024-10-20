use bevy::prelude::*;

#[derive(Bundle)]
pub struct Piece {
    pub sprite: SpriteBundle,
    pub atlas: TextureAtlas,
}