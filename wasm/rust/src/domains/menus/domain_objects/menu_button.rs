use bevy::prelude::*;

#[derive(Clone)]
pub enum MenuButtonId {
    StartSnake,
    Exit
}

#[derive(Component, Clone)]
pub struct MenuButton {
    pub id: MenuButtonId
}
