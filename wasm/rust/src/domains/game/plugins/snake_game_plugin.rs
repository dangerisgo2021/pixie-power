use crate::domains::game::domain_objects::collidable::Collidable;
use crate::domains::game::domain_objects::pickup::Pickup;
use crate::domains::game::domain_objects::piece::Piece;
use crate::domains::game::domain_objects::player::{Player, SnakeNode};
use crate::domains::game::domain_objects::snake_game::SnakeGame;
use crate::domains::game::events::{SnakeGameEvent, SnakeGameMessage};
use crate::domains::game::plugins::player_plugin::PlayerPlugin;
use crate::domains::game::services::spawn_board::spawn_board;
use crate::domains::game::value_objects::direction::Direction;
use crate::domains::game::value_objects::position::Position;
use crate::domains::graphics::resources::sprite_atlas::SpriteSheetAtlas;
use bevy::app::AppExit;
use bevy::asset::AssetServer;
use bevy::prelude::*;
use rand::{thread_rng, Rng};
use std::time::Duration;

pub struct SnakeGamePlugin;
const BACKGROUND_COLOR: Color = Color::srgb(0.9, 0.9, 0.9);
const TILE_WIDTH: f32 = 16.;
impl Plugin for SnakeGamePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SnakeGameEvent>();
        app.insert_resource(ClearColor(BACKGROUND_COLOR));
        app.insert_resource(SnakeGame {
            width: 11,
            height: 11,
            square_size: 16.,
            grid_level: -5.,
            square_color_primary: Color::linear_rgb(1.0, 0.5, 0.4),
            square_color_secondary: Color::linear_rgb(0.2, 1.0, 0.6),
            scale: 1.0,
        });
        app.insert_resource(Time::<Fixed>::from_duration(Duration::from_secs(1)));
        app.add_plugins(PlayerPlugin);
        app.add_systems(Startup, spawn_board);
        app.add_systems(Update, (handle_snake_game_events, check_collisions));
        // app.add_systems(FixedUpdate, (check_collisions, move_movables));
    }
}

fn handle_snake_game_events(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    atlas_layout: Res<SpriteSheetAtlas>,
    mut exit: EventWriter<AppExit>,
    mut snake_game_events: EventReader<SnakeGameEvent>,
    mut player_query: Query<(&mut Player, Entity), With<Player>>,
    mut pickup: Query<(&mut Pickup, &mut Transform), With<Pickup>>,
) {
    let (mut player,_) = player_query.single_mut();
    for snake_game_event in snake_game_events.read() {
        println!("snake_game_event {:?}", snake_game_event);

        match snake_game_event.event_id {
            SnakeGameMessage::StartGameCommand => {
                println!("SnakeGameMessage::StartGameCommand");
                player.direction = Direction::Right;
            }
            SnakeGameMessage::ExitGameCommand => {
                println!("SnakeGameMessage::ExitGameCommand");
                exit.send(AppExit::Success);
            }
            SnakeGameMessage::TailCollision => {
                println!("SnakeGameMessage::TailCollision");
                
                //stop the game
                
                // reset player and pickup
                
                // show menu
            }
            SnakeGameMessage::PickupCollision => {
                println!("SnakeGameMessage::PlayerCollision");
                //move pickup
                let (mut pickup, mut pickup_transform) = pickup.single_mut();
                pickup.position.x = thread_rng().gen_range(1..10);
                pickup.position.y = thread_rng().gen_range(1..10);

                pickup_transform.translation.x = pickup.position.x as f32 * TILE_WIDTH;
                pickup_transform.translation.y = pickup.position.y as f32 * TILE_WIDTH;

                //add part to tail
                commands.spawn((
                    SnakeNode {
                        position: player.position,
                        index: player.tail_length,
                    },
                    Collidable,
                    Piece {
                        sprite: SpriteBundle {
                            texture: asset_server.load("images\\fairy-spritesheet.png"),
                            transform: Transform {
                                translation: Vec3::new(player.position.x as f32 * TILE_WIDTH, player.position.y as f32 * TILE_WIDTH, -1.),
                                ..default() },
                            ..default()
                        },
                        atlas: TextureAtlas {
                            layout: atlas_layout.handle.clone(),
                            index: 3,
                        },
                    },
                ));

                player.tail_length += 1;
            }
            SnakeGameMessage::ChangePlayerDirection(direction) => {
                if player.direction != direction {
                    let (new_dir_x, new_dir_y) = direction.as_offset();
                    let (player_dir_x, player_dir_y) = player.direction.as_offset();

                    let is_opposite_dir =
                        player_dir_y + new_dir_y == 0 && player_dir_x + new_dir_x == 0;

                    if !is_opposite_dir {
                        player.direction = direction;
                    }
                }
            }
        }
    }
}
//
// fn move_movables(mut movables: Query<(&mut Movable, &mut Transform), With<Movable>>) {
//
//
//     // add speed times direction to moveable's position
//     for movable in movables.iter_mut() {
//         //move by speed in direction
//         let (mut mov, mut transform) = movable;
//
//         match mov.direction {
//             Direction::Up => {
//                 mov.position.y += 1;
//             }
//             Direction::Down => {
//                 mov.position.y -= 1;
//             }
//             Direction::Left => {
//                 mov.position.x -= 1;
//             }
//             Direction::Right => {
//                 mov.position.x += 1;
//             }
//             Direction::None => {}
//         }
//
//         transform.translation.x = mov.position.x as f32 * TILE_WIDTH;
//         transform.translation.y = mov.position.y as f32 * TILE_WIDTH;
//     }
//
// }
//
fn check_collisions(
    player: Query<&Player, With<Player>>,
    tail: Query<&SnakeNode, With<SnakeNode>>,
    pickup: Query<&Pickup, With<Pickup>>,
    mut event_writer: EventWriter<SnakeGameEvent>,
) {
    let player = player.single();
    let pickup = pickup.single();

    if player.position.x == pickup.position.x && player.position.y == pickup.position.y {
        event_writer.send(SnakeGameEvent {
            event_id: SnakeGameMessage::PickupCollision,
        });
    }
    
    for tail_node in tail.iter() {
        if player.position.x == tail_node.position.x && player.position.y == tail_node.position.y {
            event_writer.send(SnakeGameEvent {
                event_id: SnakeGameMessage::TailCollision,
            });
        }
    }
    
    
}

// add speed times direction to moveable's position

// check if player is out of bounds

// check for collisions with player

// if collision with pickup emit pickup trigger event

// if collision with wall end game

// if collision with self
