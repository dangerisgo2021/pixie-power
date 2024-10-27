use bevy::prelude::Component;
use crate::domains::game::value_objects::position::Position;

#[derive(Component)]
pub struct Wall {
    pub position: Position,
}