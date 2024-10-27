use bevy::prelude::*;
use crate::domains::game::value_objects::direction::Direction;
use crate::domains::game::value_objects::position::Position;

#[derive(Component)]
pub struct Player {
    pub tail_length: i32,
    pub position: Position,
    pub direction: Direction,
    pub can_change_direction: bool
}

#[derive(Component)]
pub struct SnakeNode {
    pub position: Position,
    pub index: i32
}