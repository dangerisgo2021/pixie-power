use bevy::prelude::*;
use crate::domains::game::value_objects::position::Position;
use crate::domains::game::value_objects::direction::Direction;

#[derive(Component)]
pub struct Movable {
    pub position: Position,
    pub direction: Direction,
    pub movement_speed: i32,
}