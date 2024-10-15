use crate::domains::game::domain_objects::movable::Movable;
use crate::domains::game::domain_objects::player::Player;
use crate::domains::game::events::{SnakeGameEvent, SnakeGameMessage};
use crate::domains::game::services::spawn_player::spawn_player;
use crate::domains::game::value_objects::direction::Direction;
use bevy::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player);
        app.add_systems(Update, player_kbd_events);
    }
}

fn player_kbd_events(
    player: Query<&Movable, With<Player>>,
    mut snake_game_event_writer: EventWriter<SnakeGameEvent>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    if keyboard_input.just_pressed(KeyCode::KeyA) || keyboard_input.just_pressed(KeyCode::ArrowLeft)
    {
        for player_movable in player.iter() {
            if player_movable.direction != Direction::Right {
                snake_game_event_writer.send(SnakeGameEvent {
                    event_id: SnakeGameMessage::ChangePlayerDirection(Direction::Left),
                });
            }
        }
    }

    if keyboard_input.just_pressed(KeyCode::KeyW) || keyboard_input.just_pressed(KeyCode::ArrowUp) {
        for player_movable in player.iter() {
            if (player_movable.direction != Direction::Down) {
                snake_game_event_writer.send(SnakeGameEvent {
                    event_id: SnakeGameMessage::ChangePlayerDirection(Direction::Up),
                });
            }
        }
    }

    if keyboard_input.just_pressed(KeyCode::KeyS) || keyboard_input.just_pressed(KeyCode::ArrowDown)
    {
        for player_movable in player.iter() {
            if (player_movable.direction != Direction::Up) {
                snake_game_event_writer.send(SnakeGameEvent {
                    event_id: SnakeGameMessage::ChangePlayerDirection(Direction::Down),
                });
            }
        }
    }

    if keyboard_input.just_pressed(KeyCode::KeyD)
        || keyboard_input.just_pressed(KeyCode::ArrowRight)
    {
        for player_movable in player.iter() {
            if (player_movable.direction != Direction::Left) {
                snake_game_event_writer.send(SnakeGameEvent {
                    event_id: SnakeGameMessage::ChangePlayerDirection(Direction::Right),
                });
            }
        }
    }
}
