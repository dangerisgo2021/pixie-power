use crate::domains::game::domain_objects::snake_game::SnakeGame;
use crate::domains::game::events::{SnakeGameEvent, SnakeGameMessage};
use crate::domains::game::value_objects::direction::Direction;

use crate::domains::menus::domain_objects::menu_button::{MenuButton, MenuButtonId};
use crate::domains::menus::events::MenuEvent;
use crate::domains::menus::services::spawn_menus::spawn_menus;
use bevy::app::{App, Plugin, Startup, Update};
use bevy::prelude::*;
use bevy::ui::Interaction;
use crate::domains::menus::domain_objects::menu_layer::{HudText, HudTextId};

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        // app.init_resource::<>(); generate menus from config
        app.add_event::<MenuEvent>();
        app.add_systems(Startup, spawn_menus);
        //app.add_plugins(FramerateCounterPlugin);
        app.add_systems(Update, menu_interactions_event_emitter);
        app.add_systems(Update, menu_event_handler);
        app.add_systems(Update, update_hud);
    }
}

fn update_hud(snake_game: Res<SnakeGame>, mut hud_text: Query<(&mut Text, &HudText), With<HudText>>) {

    for (mut text, hud_text) in &mut hud_text {
        match hud_text.id {
            HudTextId::current_score => {
                text.sections[0].value = format!("{}", snake_game.current_score);
            }
            HudTextId::high_score => {
                text.sections[0].value = format!("{}", snake_game.high_score);
            }
        }

    }

}

fn menu_event_handler(
    mut menu_event_reader: EventReader<MenuEvent>,
    mut snake_game_event_writer: EventWriter<SnakeGameEvent>,
) {
    for menu_event in menu_event_reader.read() {
        match menu_event.menu_button_id {
            MenuButtonId::StartSnake => {
                println!("Starting snake");

                // emit start snake command
                snake_game_event_writer.send(SnakeGameEvent {
                    event_id: SnakeGameMessage::StartGameCommand,
                });
            }
            MenuButtonId::Exit => {
                println!("Exit");
                // exit game command
                snake_game_event_writer.send(SnakeGameEvent {
                    event_id: SnakeGameMessage::ExitGameCommand,
                });
            }
            MenuButtonId::ChangeRight => {
                snake_game_event_writer.send(SnakeGameEvent {
                    event_id: SnakeGameMessage::ChangePlayerDirection(Direction::Right),
                });
            }
            MenuButtonId::ChangeLeft => {
                snake_game_event_writer.send(SnakeGameEvent {
                    event_id: SnakeGameMessage::ChangePlayerDirection(Direction::Left),
                });
            }
            MenuButtonId::ChangeUp => {
                snake_game_event_writer.send(SnakeGameEvent {
                    event_id: SnakeGameMessage::ChangePlayerDirection(Direction::Up),
                });
            }
            MenuButtonId::ChangeDown => {
                snake_game_event_writer.send(SnakeGameEvent {
                    event_id: SnakeGameMessage::ChangePlayerDirection(Direction::Down),
                });
            }
        }
    }
}

fn menu_interactions_event_emitter(
    mut event_writer: EventWriter<MenuEvent>,
    interaction_query: Query<(&Interaction, &MenuButton), Changed<Interaction>>,
) {
    for (interaction, menu_button) in interaction_query.iter() {
        match interaction {
            Interaction::Pressed => {
                event_writer.send(MenuEvent {
                    menu_button_id: menu_button.id.clone(),
                    interaction: *interaction,
                });
            }
            Interaction::Hovered => {}
            Interaction::None => {}
        }
    }
}
