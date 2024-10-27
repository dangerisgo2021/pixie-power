use bevy::prelude::*;

#[derive(Bundle)]
pub struct SpriteWithAtlas {
    pub sprite: SpriteBundle,
    pub atlas: TextureAtlas,
}