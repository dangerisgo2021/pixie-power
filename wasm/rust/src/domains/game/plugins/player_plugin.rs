use crate::domains::game::domain_objects::player::{Player, SnakeNode};
use crate::domains::game::domain_objects::snake_game::{SnakeGame, SnakeGameState};
use crate::domains::game::events::{SnakeGameEvent, SnakeGameMessage};
use crate::domains::game::services::spawn_player::spawn_player;
use crate::domains::game::value_objects::direction::Direction;
use bevy::prelude::*;
use bevy::utils::HashMap;
use std::time::Duration;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Time::<Fixed>::from_duration(Duration::from_millis(1000)));
        app.add_systems(Startup, spawn_player);
        app.add_systems(Update, player_kbd_events.run_if(in_state(SnakeGameState::Running)));
        app.add_systems(FixedUpdate, move_player.run_if(in_state(SnakeGameState::Running)));
    }
}

fn move_player(
    mut player_query: Query<(&mut Player, &mut Transform), With<Player>>,
    mut tail_query: Query<
        (&mut SnakeNode, &mut Transform),
        (With<SnakeNode>, Without<Player>),
    >,
    snake_game: Res<SnakeGame>,
) {
    let (mut player, mut player_transform) = player_query.single_mut();

    if player.direction == Direction::None {
        return;
    }
    player.can_change_direction = true;
    let tail: Vec<(&SnakeNode, &Transform)> = tail_query.iter().collect();

    let mut parent_pos_map = HashMap::new();
    tail.iter().for_each(|(cur_snake_node, _)| {
        let parent_node = tail_query
            .iter()
            .find(|(snake_node, _)| snake_node.index == cur_snake_node.index - 1);

        match parent_node {
            Some((parent_snake_node, _)) => {
                parent_pos_map.insert(cur_snake_node.index, parent_snake_node.position);
            }
            None => {
                parent_pos_map.insert(cur_snake_node.index, player.position);
            }
        }
    });
    for tail_node in tail_query.iter_mut() {
        let (mut cur_snake_node, mut snake_node_transform) = tail_node;
        let parent_position = parent_pos_map.get(&cur_snake_node.index).unwrap();

        cur_snake_node.position = *parent_position;
        snake_node_transform.translation.x =
            cur_snake_node.position.x as f32 * snake_game.square_size;
        snake_node_transform.translation.y =
            cur_snake_node.position.y as f32 * snake_game.square_size;
        
    }
    
    match player.direction {
        Direction::Up => {
            player.position.y += 1;
        }
        Direction::Down => {
            player.position.y -= 1;
        }
        Direction::Left => {
            player.position.x -= 1;
        }
        Direction::Right => {
            player.position.x += 1;
        }
        Direction::None => {}
    }

    player_transform.translation.x = player.position.x as f32 * snake_game.square_size;
    player_transform.translation.y = player.position.y as f32 * snake_game.square_size;
    
}

fn player_kbd_events(
    player: Query<&Player, With<Player>>,
    mut snake_game_event_writer: EventWriter<SnakeGameEvent>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    let player = player.single();
    
    if keyboard_input.pressed(KeyCode::Escape) {
        snake_game_event_writer.send(SnakeGameEvent {
            event_id: SnakeGameMessage::PauseGame
        });
    }
    
    if (keyboard_input.just_pressed(KeyCode::KeyA)
        || keyboard_input.just_pressed(KeyCode::ArrowLeft))
        && player.direction != Direction::Right
    {
        snake_game_event_writer.send(SnakeGameEvent {
            event_id: SnakeGameMessage::ChangePlayerDirection(Direction::Left),
        });
    }

    if (keyboard_input.just_pressed(KeyCode::KeyW) || keyboard_input.just_pressed(KeyCode::ArrowUp))
        && (player.direction != Direction::Down)
    {
        snake_game_event_writer.send(SnakeGameEvent {
            event_id: SnakeGameMessage::ChangePlayerDirection(Direction::Up),
        });
    }

    if (keyboard_input.just_pressed(KeyCode::KeyS)
        || keyboard_input.just_pressed(KeyCode::ArrowDown))
        && (player.direction != Direction::Up)
    {
        snake_game_event_writer.send(SnakeGameEvent {
            event_id: SnakeGameMessage::ChangePlayerDirection(Direction::Down),
        });
    }

    if (keyboard_input.just_pressed(KeyCode::KeyD)
        || keyboard_input.just_pressed(KeyCode::ArrowRight))
        && (player.direction != Direction::Left)
    {
        snake_game_event_writer.send(SnakeGameEvent {
            event_id: SnakeGameMessage::ChangePlayerDirection(Direction::Right),
        });
    }
    if (keyboard_input.just_pressed(KeyCode::KeyD)
        || keyboard_input.just_pressed(KeyCode::ArrowRight))
        && (player.direction != Direction::Left)
    {
        snake_game_event_writer.send(SnakeGameEvent {
            event_id: SnakeGameMessage::ChangePlayerDirection(Direction::Right),
        });
    }
}
