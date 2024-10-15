use bevy::prelude::{Event, Interaction};
use crate::domains::menus::domain_objects::menu_button::MenuButtonId;

#[derive(Event)]
pub struct MenuEvent {
    pub menu_button_id: MenuButtonId,
    pub interaction: Interaction
}