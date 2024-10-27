use crate::domains::game::domain_objects::sprite_with_atlas::SpriteWithAtlas;
use crate::domains::graphics::resources::sprite_atlas::SpriteSheetAtlas;
use crate::domains::menus::domain_objects::menu_button::{MenuButton, MenuButtonId};
use crate::domains::menus::domain_objects::menu_layer::{
    ControlsLayer, HudLayer, HudText, MenuLayer,
};
use crate::domains::menus::domain_objects::menu_text::MenuText;
use bevy::prelude::*;

pub fn spawn_menus(
    mut commands: Commands,
    // asset_server: Res<AssetServer>,
    // atlas_layout: Res<SpriteSheetAtlas>,
) {
    // main menu
    commands
        .spawn((
            MenuLayer,
            NodeBundle {
                background_color: BackgroundColor(Color::BLACK),
                z_index: ZIndex::Global(10000),
                style: Style {
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    width: Val::Percent(100.),
                    height: Val::Percent(100.),
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
                            height: Val::Auto,
                            border: UiRect::all(Val::Px(5.0)),
                            // horizontally center child text
                            justify_content: JustifyContent::Center,
                            // vertically center child text
                            align_items: AlignItems::Center,
                            padding: UiRect::all(Val::Px(20.0)),
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
                            padding: UiRect::all(Val::Px(20.0)),
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

    //hud layer

    commands
        .spawn((
            HudLayer,
            NodeBundle {
                background_color: BackgroundColor(Color::BLACK),
                z_index: ZIndex::Global(1000),
                style: Style {
                    align_items: AlignItems::Center,
                    width: Val::Percent(100.),
                    height: Val::Px(100.),
                    // give it some padding for readability
                    padding: UiRect::all(Val::Px(4.0)),
                    ..default()
                },
                ..default()
            },
        ))
        .with_children(|parent| {
            //Current Pickups
            parent.spawn((
                HudText,
                TextBundle {
                    text: Text::from_sections([
                        TextSection {
                            value: "Current: ".into(),
                            style: TextStyle {
                                font_size: 16.0,
                                color: Color::WHITE,
                                ..default()
                            },
                        },
                        TextSection {
                            value: " 0".into(),
                            style: TextStyle {
                                font_size: 16.0,
                                color: Color::WHITE,
                                ..default()
                            },
                        },
                    ]),
                    ..default()
                },
            ));
            //Highscore
            parent.spawn((
                HudText,
                TextBundle {
                    text: Text::from_sections([
                        TextSection {
                            value: "Highscore: ".into(),
                            style: TextStyle {
                                font_size: 16.0,
                                color: Color::WHITE,
                                ..default()
                            },
                        },
                        TextSection {
                            value: " 0".into(),
                            style: TextStyle {
                                font_size: 16.0,
                                color: Color::WHITE,
                                ..default()
                            },
                        },
                    ]),
                    ..default()
                },
            ));
        });

    //controls layer

    commands
        .spawn((
            ControlsLayer,
            NodeBundle {
                background_color: BackgroundColor(Color::BLACK.with_alpha(0.75)),
                z_index: ZIndex::Global(1000),
                style: Style {
                    align_items: AlignItems::Center,
                    width: Val::Percent(100.),
                    height: Val::Px(100.),
                    top: Val::Px(500.0),
                    justify_content: JustifyContent::SpaceAround,
                    // give it some padding for readability
                    padding: UiRect::all(Val::Px(4.0)),
                    ..default()
                },
                ..default()
            },
        ))
        .with_children(|parent| {
            //direction controls left
            parent
                .spawn((
                    MenuButton {
                        id: MenuButtonId::ChangeLeft,
                    },
                    ButtonBundle {
                        style: Style {
                            padding: UiRect::all(Val::Px(10.0)),
                            align_content: AlignContent::Center,
                            justify_content: JustifyContent::Center,
                            ..default()
                        },
                        background_color: BackgroundColor(Color::BLACK.with_alpha(0.75)),
                        border_color: BorderColor(Color::BLACK.with_alpha(0.25)),
                        border_radius: BorderRadius::all(Val::Px(4.0)),

                        ..default()
                    },
                ))
                .with_children(|parent| {
                    parent.spawn((
                        MenuText,
                        TextBundle {
                            text: Text::from_section(
                                // Accepts a String or any type that converts into a String, such as &str.
                                "<",
                                TextStyle {
                                    color: Color::WHITE,
                                    font_size: 20.,
                                    ..default()
                                },
                            ),
                            ..default()
                        },
                    ));
                });
            //direction controls up

            parent
                .spawn((
                    MenuButton {
                        id: MenuButtonId::ChangeUp,
                    },
                    ButtonBundle {
                        style: Style {
                            padding: UiRect::all(Val::Px(10.0)),
                            align_content: AlignContent::Center,
                            justify_content: JustifyContent::Center,
                            ..default()
                        },
                        background_color: BackgroundColor(Color::BLACK.with_alpha(0.75)),
                        border_color: BorderColor(Color::BLACK.with_alpha(0.25)),
                        border_radius: BorderRadius::all(Val::Px(4.0)),
                        ..default()
                    },
                ))
                .with_children(|parent| {
                    parent.spawn((
                        MenuText,
                        TextBundle {
                            text: Text::from_section(
                                // Accepts a String or any type that converts into a String, such as &str.
                                "^",
                                TextStyle {
                                    color: Color::WHITE,
                                    font_size: 20.,
                                    ..default()
                                },
                            ),
                            ..default()
                        },
                    ));
                });

            //direction controls down

            parent
                .spawn((
                    MenuButton {
                        id: MenuButtonId::ChangeDown,
                    },
                    ButtonBundle {
                        style: Style {
                            padding: UiRect::all(Val::Px(10.0)),
                            align_content: AlignContent::Center,
                            justify_content: JustifyContent::Center,
                            ..default()
                        },
                        background_color: BackgroundColor(Color::BLACK.with_alpha(0.75)),
                        border_color: BorderColor(Color::BLACK.with_alpha(0.25)),
                        border_radius: BorderRadius::all(Val::Px(4.0)),
                        ..default()
                    },
                ))
                .with_children(|parent| {
                    parent.spawn((
                        MenuText,
                        TextBundle {
                            text: Text::from_section(
                                // Accepts a String or any type that converts into a String, such as &str.
                                "v",
                                TextStyle {
                                    color: Color::WHITE,
                                    font_size: 20.,
                                    ..default()
                                },
                            ),
                            ..default()
                        },
                    ));
                });

            //direction controls right
            parent
                .spawn((
                    MenuButton {
                        id: MenuButtonId::ChangeRight,
                    },
                    ButtonBundle {
                        style: Style {
                            padding: UiRect::all(Val::Px(10.0)),
                            align_content: AlignContent::Center,
                            justify_content: JustifyContent::Center,
                            ..default()
                        },
                        background_color: BackgroundColor(Color::BLACK.with_alpha(0.75)),
                        border_color: BorderColor(Color::BLACK.with_alpha(0.25)),
                        border_radius: BorderRadius::all(Val::Px(4.0)),
                        ..default()
                    },
                ))
                .with_children(|parent| {
                    parent.spawn((
                        MenuText,
                        TextBundle {
                            text: Text::from_section(
                                // Accepts a String or any type that converts into a String, such as &str.
                                ">",
                                TextStyle {
                                    color: Color::WHITE,
                                    font_size: 20.,
                                    ..default()
                                },
                            ),
                            ..default()
                        },
                    ));
                });
        });
}
