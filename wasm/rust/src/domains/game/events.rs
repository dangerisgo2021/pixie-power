use bevy::prelude::*;
use crate::domains::game::value_objects::direction::Direction;

#[derive(Debug)]
pub enum SnakeGameMessage {
    StartGameCommand,
    ExitGameCommand,
    ChangePlayerDirection(Direction),
    PickupCollision,
    TailCollision,
}

#[derive(Event, Debug)]
pub struct SnakeGameEvent {
  pub event_id: SnakeGameMessage,
}
