use bevy::prelude::*;

#[derive(Component)]
pub struct MenuLayer;

#[derive(Component)]
pub struct HudLayer;

pub enum HudTextId {
    current_score,
    high_score,
}
#[derive(Component)]
pub struct HudText {
    pub id: HudTextId,
}

#[derive(Component)]
pub struct ControlsLayer;