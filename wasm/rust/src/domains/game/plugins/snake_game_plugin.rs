use crate::domains::game::domain_objects::collidable::Collidable;
use crate::domains::game::domain_objects::pickup::Pickup;
use crate::domains::game::domain_objects::player::{Player, SnakeNode};
use crate::domains::game::domain_objects::snake_game::{SnakeGame, SnakeGameState};
use crate::domains::game::domain_objects::sprite_with_atlas::SpriteWithAtlas;
use crate::domains::game::domain_objects::wall::Wall;
use crate::domains::game::events::{SnakeGameEvent, SnakeGameMessage};
use crate::domains::game::plugins::player_plugin::PlayerPlugin;
use crate::domains::game::services::spawn_board::spawn_board;
use crate::domains::game::value_objects::direction::Direction;
use crate::domains::game::value_objects::position::Position;
use crate::domains::graphics::resources::sprite_atlas::SpriteSheetAtlas;
use crate::domains::menus::domain_objects::menu_layer::MenuLayer;
use bevy::app::AppExit;
use bevy::asset::AssetServer;
use bevy::prelude::*;
use rand::{thread_rng, Rng};

pub struct SnakeGamePlugin;
const BACKGROUND_COLOR: Color = Color::srgb(0.9, 0.9, 0.9);
impl Plugin for SnakeGamePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SnakeGameEvent>();
        app.insert_state(SnakeGameState::Paused);
        app.insert_resource(ClearColor(Color::srgb(56. / 256., 105. / 256.,0.0)));
        app.insert_resource(SnakeGame {
            high_score: 0,
            current_score: 0,
            width: 8,
            height: 8,
            square_size: 16.,
            grid_level: -5.,
            square_color_primary: Color::linear_rgb(1.0, 0.5, 0.4),
            square_color_secondary: Color::linear_rgb(0.2, 1.0, 0.6),
            scale: 1.0,
        });
        app.add_plugins(PlayerPlugin);
        app.add_systems(Startup, spawn_board);
        app.add_systems(Update, (handle_snake_game_events, check_collisions));
    }
}

fn handle_snake_game_events(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    atlas_layout: Res<SpriteSheetAtlas>,
    mut game_state: ResMut<NextState<SnakeGameState>>,
    mut exit: EventWriter<AppExit>,
    mut snake_game: ResMut<SnakeGame>,
    mut snake_game_events: EventReader<SnakeGameEvent>,
    mut player_query: Query<(&mut Player, Entity), With<Player>>,
    mut pickup: Query<(&mut Pickup, &mut Transform), With<Pickup>>,
    mut snake_node: Query<Entity, With<SnakeNode>>,
    mut menu_query: Query<(&mut MenuLayer, &mut Visibility), With<MenuLayer>>,
) {
    let (mut player, _) = player_query.single_mut();

    for snake_game_event in snake_game_events.read() {
        println!("snake_game_event {:?}", snake_game_event);

        match snake_game_event.event_id {
            SnakeGameMessage::StartGameCommand => {
                println!("SnakeGameMessage::StartGameCommand");
                player.direction = Direction::Right;
                let (_, mut menu_vis) = menu_query.single_mut();
                *menu_vis = Visibility::Hidden;
                game_state.set(SnakeGameState::Running);

                snake_game.current_score = 0;

                for tail in snake_node.iter() {
                    commands.entity(tail).despawn();
                }
            }
            SnakeGameMessage::ExitGameCommand => {
                println!("SnakeGameMessage::ExitGameCommand");
                exit.send(AppExit::Success);
            }
            SnakeGameMessage::PauseGame => {
                println!("SnakeGameMessage::PauseGame");
                let (_, mut menu_vis) = menu_query.single_mut();
                *menu_vis = Visibility::Visible;
                game_state.set(SnakeGameState::Paused);
            }
            SnakeGameMessage::TailCollision => {
                println!("SnakeGameMessage::TailCollision");

                //stop the game
                let (_, mut menu_vis) = menu_query.single_mut();
                *menu_vis = Visibility::Visible;
                game_state.set(SnakeGameState::Paused);
                player.position = Position { x: 3, y: 3 };
            }
            SnakeGameMessage::WallCollision => {
                println!("SnakeGameMessage::WallCollision");

                //stop the game
                let (_, mut menu_vis) = menu_query.single_mut();
                *menu_vis = Visibility::Visible;
                game_state.set(SnakeGameState::Paused);
                player.position = Position { x: 3, y: 3 };
            }
            SnakeGameMessage::PickupCollision => {
                println!("SnakeGameMessage::PlayerCollision");
                //move pickup
                let (mut pickup, mut pickup_transform) = pickup.single_mut();
                pickup.position.x = thread_rng().gen_range(2..7);
                pickup.position.y = thread_rng().gen_range(2..7);

                pickup_transform.translation.x = pickup.position.x as f32 * snake_game.square_size;
                pickup_transform.translation.y = pickup.position.y as f32 * snake_game.square_size;

                //add part to tail
                commands.spawn((
                    SnakeNode {
                        position: Position { x: -10, y: -10 },
                        index: player.tail_length,
                    },
                    Collidable,
                    SpriteWithAtlas {
                        sprite: SpriteBundle {
                            texture: asset_server.load("images\\fairy-spritesheet.png"),
                            transform: Transform {
                                translation: Vec3::new(
                                    player.position.x as f32 * snake_game.square_size,
                                    player.position.y as f32 * snake_game.square_size,
                                    -1.,
                                ),
                                ..default()
                            },
                            ..default()
                        },
                        atlas: TextureAtlas {
                            layout: atlas_layout.handle.clone(),
                            index: 3,
                        },
                    },
                ));
                player.tail_length += 1;
                snake_game.current_score += 1;

                if(snake_game.high_score < snake_game.current_score) {
                    snake_game.high_score = snake_game.current_score;
                }
            }
            SnakeGameMessage::ChangePlayerDirection(direction) => {
                if player.can_change_direction {
                    player.can_change_direction = false;
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
}

fn check_collisions(
    player: Query<&Player, With<Player>>,
    tail: Query<&SnakeNode, With<SnakeNode>>,
    walls: Query<&Wall, With<Wall>>,
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

    for wall_node in walls.iter() {
        if player.position.x == wall_node.position.x && player.position.y == wall_node.position.y {
            event_writer.send(SnakeGameEvent {
                event_id: SnakeGameMessage::WallCollision,
            });
        }
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
