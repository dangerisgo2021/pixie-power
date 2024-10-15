use bevy::prelude::*;
use crate::domains::game::value_objects::position::Position;

#[derive(Component)]
pub struct GridSquare {
    pub position: Position,
}