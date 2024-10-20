use crate::domains::menus::domain_objects::menu_button::{MenuButton, MenuButtonId};
use crate::domains::menus::events::MenuEvent;
use crate::domains::menus::services::spawn_menus::spawn_menus;
use bevy::app::{App, Plugin, Startup, Update};
use bevy::prelude::*;
use bevy::ui::Interaction;
use crate::domains::game::events::{SnakeGameEvent, SnakeGameMessage};

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        // app.init_resource::<>(); generate menus from config
        app.add_event::<MenuEvent>();
        app.add_systems(Startup, spawn_menus);
        //app.add_plugins(FramerateCounterPlugin);
        app.add_systems(Update, menu_interactions_event_emitter);
        app.add_systems(Update, menu_event_handler);
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
                // hide menu
                
                // emit start snake command
                snake_game_event_writer.send( SnakeGameEvent {
                    event_id: SnakeGameMessage::StartGameCommand
                });
            },
            MenuButtonId::Exit => {
                println!("Exit");
                // exit game command
                snake_game_event_writer.send( SnakeGameEvent {
                    event_id: SnakeGameMessage::ExitGameCommand
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
