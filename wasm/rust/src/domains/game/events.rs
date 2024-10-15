use bevy::prelude::*;
use crate::domains::game::value_objects::direction::Direction;

pub enum SnakeGameMessage {
    StartGameCommand,
    ExitGameCommand,
    ChangePlayerDirection(Direction),
}

#[derive(Event)]
pub struct SnakeGameEvent {
  pub event_id: SnakeGameMessage,
}
