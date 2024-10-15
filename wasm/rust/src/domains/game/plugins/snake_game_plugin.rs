use crate::domains::game::domain_objects::collidable::Collidable;
use crate::domains::game::domain_objects::movable::Movable;
use crate::domains::game::domain_objects::pickup::Pickup;
use crate::domains::game::domain_objects::piece::Piece;
use crate::domains::game::domain_objects::player::Player;
use crate::domains::game::events::{SnakeGameEvent, SnakeGameMessage};
use crate::domains::game::services::spawn_board::spawn_board;
use crate::domains::game::value_objects::direction::Direction;
use crate::domains::game::value_objects::position::Position;
use crate::domains::graphics::resources::sprite_atlas::SpriteSheetAtlas;
use bevy::asset::AssetServer;
use bevy::prelude::{
    default, App, AppExit, ClearColor, Color, Commands, EventReader, EventWriter,
    IntoSystemConfigs, Plugin, Query, Res, SpriteBundle, Startup, TextureAtlas, Time, Transform,
    Update, With,
};

pub struct SnakeGamePlugin;
const BACKGROUND_COLOR: Color = Color::srgb(0.9, 0.9, 0.9);
const TILE_WIDTH: f32 = 16.;
impl Plugin for SnakeGamePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SnakeGameEvent>();
        app.insert_resource(ClearColor(BACKGROUND_COLOR));
        app.add_systems(Startup, spawn_board);
        app.add_systems(Update, (handle_snake_game_events, move_movables, check_collisions).chain());
    }
}

fn handle_snake_game_events(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    atlas_layout: Res<SpriteSheetAtlas>,
    mut exit: EventWriter<AppExit>,
    mut snake_game_events: EventReader<SnakeGameEvent>,
    mut player: Query<&mut Movable, With<Player>>,
) {
    for snake_game_event in snake_game_events.read() {
        match snake_game_event.event_id {
            SnakeGameMessage::StartGameCommand => {
                println!("SnakeGameMessage::StartGameCommand");
                for mut movable in player.iter_mut() {
                    movable.direction = Direction::Right;
                    movable.movement_speed = 32;
                }
                commands.spawn((
                    Pickup,
                    Piece {
                        position: Position { x: 0, y: 0 },
                        sprite: SpriteBundle {
                            texture: asset_server.load("images\\fairy-spritesheet.png"),
                            ..default()
                        },
                        atlas: TextureAtlas {
                            layout: atlas_layout.handle.clone(),
                            index: 0,
                        },
                    },
                ));
            }
            SnakeGameMessage::ExitGameCommand => {
                println!("SnakeGameMessage::ExitGameCommand");
                exit.send(AppExit::Success);
            }
            SnakeGameMessage::ChangePlayerDirection(direction) => {
                for mut movable in player.iter_mut() {
                    if (movable.direction != direction) {
                        let (new_dir_x, new_dir_y) = direction.as_offset();
                        let (player_dir_x, player_dir_y) = movable.direction.as_offset();

                        let is_opposite_dir =
                            player_dir_y + new_dir_y == 0 && player_dir_x + new_dir_x == 0;

                        if (!is_opposite_dir) {
                            movable.direction = direction;
                        }
                    }
                }
            }
        }
    }
}

fn move_movables(time: Res<Time>, mut movables: Query<(&mut Movable, &mut Transform), With<Movable>>) {
    // add speed times direction to moveable's position
    for movable in movables.iter_mut() {
        
        //move by speed in direction
        let (mut mov, mut transform) = movable;
        // let (dir_x, dir_y) = mov.direction.as_offset();
        // transform.translation.x += (mov.movement_speed * dir_x) as f32 * time.delta_seconds();
        // transform.translation.y += (mov.movement_speed * dir_y) as f32 * time.delta_seconds();
        // 
        // calculate position based on new position
        let new_pos_x = transform.translation.x / TILE_WIDTH;
        let new_pos_y = transform.translation.y / TILE_WIDTH;

        mov.position = Position::new(new_pos_x.ceil() as i32, new_pos_y.ceil() as i32);
        // clamp perpendicular direction to center of tile grid
        println!("movable position: {:?} no floor: [{}, {}]", mov.position, new_pos_x, new_pos_y);
        match mov.direction {
            Direction::Up => {
                transform.translation.y += (mov.movement_speed) as f32 * time.delta_seconds();
                transform.translation.x = new_pos_x.floor() * TILE_WIDTH;
            }
            Direction::Down => {
                transform.translation.y -= (mov.movement_speed) as f32 * time.delta_seconds();
                transform.translation.x = new_pos_x.ceil() * TILE_WIDTH;
            }
            Direction::Left => {
                transform.translation.x -= (mov.movement_speed) as f32 * time.delta_seconds();
                transform.translation.y = new_pos_y.ceil() * TILE_WIDTH;

            }
            Direction::Right => {
                transform.translation.x += (mov.movement_speed) as f32 * time.delta_seconds();
                transform.translation.y = new_pos_y.ceil() * TILE_WIDTH;
            }
            Direction::None => {}
        }
    }
    // check if player is out of bounds

    // check for collisions with player

    // if collision with pickup emit pickup trigger event

    // if collision with wall end game

    // if collision with self
}

fn check_collisions(
    mut collidables: Query<(&Movable, &mut Transform), With<Collidable>>,
) {
    // add speed times direction to moveable's position
    for movable in collidables.iter_mut() {}
    // check if player is out of bounds

    // check for collisions with player

    // if collision with pickup emit pickup trigger event

    // if collision with wall end game

    // if collision with self
}
