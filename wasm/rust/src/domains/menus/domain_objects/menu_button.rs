use bevy::prelude::*;

#[derive(Clone)]
pub enum MenuButtonId {
    StartSnake,
    Exit,
    ChangeRight,
    ChangeLeft,
    ChangeUp,
    ChangeDown,
}

#[derive(Component, Clone)]
pub struct MenuButton {
    pub id: MenuButtonId
}
