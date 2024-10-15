use crate::domains::menus::domain_objects::menu_button::{MenuButton, MenuButtonId};
use crate::domains::menus::domain_objects::menu_layer::MenuLayer;
use crate::domains::menus::domain_objects::menu_text::MenuText;
use bevy::prelude::*;

pub fn spawn_menus(mut commands: Commands) {
    commands
        .spawn((
            MenuLayer,
            NodeBundle {
                background_color: BackgroundColor(Color::BLACK),
                z_index: ZIndex::Global(10000),
                style: Style {
                    flex_direction: FlexDirection::Column,
                    position_type: PositionType::Absolute,
                    // position it in the top-right corner
                    // 1% away from the top window edge
                    left: Val::Percent(1.),
                    top: Val::Percent(1.),
                    // set bottom/left to Auto, so it can be
                    // automatically sized depending on the text
                    bottom: Val::Auto,
                    right: Val::Auto,
                    // give it some padding for readability
                    padding: UiRect::all(Val::Px(4.0)),
                    ..default()
                },
                ..default()
            },
        ))
        .with_children(|parent| {
            //Menu Title
            parent.spawn((
                MenuText,
                TextBundle {
                    text: Text::from_section(
                        // Accepts a String or any type that converts into a String, such as &str.
                        "Pixie Power",
                        TextStyle {
                            font_size: 60.0,
                            color: Color::WHITE,
                            ..default()
                        },
                    ),
                    ..default()
                },
            ));
            //start button
            parent
                .spawn((
                    MenuButton {
                        id: MenuButtonId::StartSnake,
                    },
                    ButtonBundle {
                        style: Style {
                            height: Val::Px(65.0),
                            border: UiRect::all(Val::Px(5.0)),
                            // horizontally center child text
                            justify_content: JustifyContent::Center,
                            // vertically center child text
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        background_color: BackgroundColor(Color::WHITE.with_alpha(0.5)),
                        border_color: BorderColor(Color::BLACK),
                        border_radius: BorderRadius::MAX,
                        ..default()
                    },
                ))
                .with_children(|parent| {
                    parent.spawn((
                        MenuText,
                        TextBundle {
                            text: Text::from_section(
                                // Accepts a String or any type that converts into a String, such as &str.
                                "Start Snake Game",
                                TextStyle {
                                    color: Color::WHITE,
                                    ..default()
                                },
                            ),
                            ..default()
                        },
                    ));
                });

            //exit button
            parent
                .spawn((
                    MenuButton {
                        id: MenuButtonId::Exit,
                    },
                    ButtonBundle {
                        style: Style {
                            height: Val::Px(65.0),
                            border: UiRect::all(Val::Px(5.0)),
                            // horizontally center child text
                            justify_content: JustifyContent::Center,
                            // vertically center child text
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        background_color: BackgroundColor(Color::WHITE.with_alpha(0.5)),
                        border_color: BorderColor(Color::BLACK),
                        border_radius: BorderRadius::MAX,
                        ..default()
                    },
                ))
                .with_children(|parent| {
                    parent.spawn((
                        MenuText,
                        TextBundle {
                            text: Text::from_section(
                                // Accepts a String or any type that converts into a String, such as &str.
                                "Exit",
                                TextStyle {
                                    color: Color::WHITE,
                                    ..default()
                                },
                            ),
                            ..default()
                        },
                    ));
                });
        });
}
