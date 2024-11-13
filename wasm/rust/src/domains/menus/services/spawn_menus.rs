use crate::domains::graphics::resources::sprite_atlas::SpriteSheetAtlas;
use crate::domains::menus::domain_objects::menu_button::{MenuButton, MenuButtonId};
use crate::domains::menus::domain_objects::menu_layer::{
    ControlsLayer, HudLayer, HudText, HudTextId, MenuLayer,
};
use crate::domains::menus::domain_objects::menu_text::MenuText;
use bevy::prelude::*;

pub fn spawn_menus(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    atlas_layout: Res<SpriteSheetAtlas>,
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
                    align_items: AlignItems::Center,
                    width: Val::Percent(100.),
                    height: Val::Percent(100.),
                    // give it some padding for readability
                    ..default()
                },
                ..default()
            },
            UiImage::new(asset_server.load("images\\fairy-house.png")),
        ))
        .with_children(|parent| {
            //Menu Title
            // parent.spawn((
            //     MenuText,
            //     TextBundle {
            //         text: Text::from_section(
            //             // Accepts a String or any type that converts into a String, such as &str.
            //             "Pixie Power",
            //             TextStyle {
            //                 font_size: 60.0,
            //                 color: Color::WHITE,
            //                 ..default()
            //             },
            //         ),
            //         ..default()
            //     },
            // ));
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
                            margin: UiRect::top(Val::Px(100.0)),
                            ..default()
                        },
                        background_color: BackgroundColor(Color::WHITE.with_alpha(0.75)),
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
                                "Collect Crystals",
                                TextStyle {
                                    color: Color::BLACK,
                                    ..default()
                                },
                            ),
                            ..default()
                        },
                    ));
                });

            //exit button
            // parent
            //     .spawn((
            //         MenuButton {
            //             id: MenuButtonId::Exit,
            //         },
            //         ButtonBundle {
            //             style: Style {
            //                 height: Val::Px(65.0),
            //                 border: UiRect::all(Val::Px(5.0)),
            //                 // horizontally center child text
            //                 justify_content: JustifyContent::Center,
            //                 // vertically center child text
            //                 align_items: AlignItems::Center,
            //                 padding: UiRect::all(Val::Px(20.0)),
            //                 ..default()
            //             },
            //             background_color: BackgroundColor(Color::WHITE.with_alpha(0.5)),
            //             border_color: BorderColor(Color::BLACK),
            //             border_radius: BorderRadius::MAX,
            //             ..default()
            //         },
            //     ))
            //     .with_children(|parent| {
            //         parent.spawn((
            //             MenuText,
            //             TextBundle {
            //                 text: Text::from_section(
            //                     // Accepts a String or any type that converts into a String, such as &str.
            //                     "Exit",
            //                     TextStyle {
            //                         color: Color::WHITE,
            //                         ..default()
            //                     },
            //                 ),
            //                 ..default()
            //             },
            //         ));
            //     });
        });

    //hud layer

    commands
        .spawn((
            HudLayer,
            NodeBundle {
                z_index: ZIndex::Global(10050),
                style: Style {
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::SpaceEvenly,
                    width: Val::Percent(100.),
                    margin: UiRect::top(Val::Px(16.0)),
                    ..default()
                },
                ..default()
            },
        ))
        .with_children(|parent| {
            parent
                .spawn(
                    (NodeBundle {
                        background_color: BackgroundColor(Color::hsv(0.88, 0.27, 1.)),
                        border_color: BorderColor(Color::BLACK),
                        border_radius: BorderRadius::all(Val::Px(3.0)),
                        style: Style {
                            width: Val::Percent(20.),
                            border: UiRect::all(Val::Px(3.0)),
                            // vertically center child text
                            align_items: AlignItems::Start,
                            justify_content: JustifyContent::Center,
                            padding: UiRect::all(Val::Px(8.0)),
                            ..default()
                        },
                        ..default()
                    }),
                )
                .with_children(|parent| {
                    parent.spawn((
                        ImageBundle {
                            image: UiImage::new(asset_server.load("images\\fairy-spritesheet.png")),
                            style: Style {
                                width: Val::Px(32.),
                                height: Val::Px(32.),
                                ..default()
                            },
                            ..default()
                        },
                        TextureAtlas {
                            layout: atlas_layout.handle.clone(),
                            index: 3,
                        },
                    ));
                    parent.spawn((
                        HudText {
                            id: HudTextId::current_score,
                        },
                        TextBundle {
                            text: Text::from_sections([TextSection {
                                value: " 0".into(),
                                style: TextStyle {
                                    font_size: 32.0,
                                    color: Color::BLACK,
                                    ..default()
                                },
                            }]),
                            ..default()
                        },
                    ));
                });

            //highscore
            parent
                .spawn(
                    (NodeBundle {
                        background_color: BackgroundColor(Color::linear_rgb(0.63, 0.44, 1.)),
                        border_color: BorderColor(Color::BLACK),
                        style: Style {
                            width: Val::Percent(20.),
                            border: UiRect::all(Val::Px(3.0)),
                            align_items: AlignItems::Start,
                            justify_content: JustifyContent::Center,
                            padding: UiRect::all(Val::Px(8.0)),
                            ..default()
                        },
                        ..default()
                    }),
                )
                .with_children(|parent| {
                    parent.spawn((
                        ImageBundle {
                            image: UiImage::new(asset_server.load("images\\fairy-spritesheet.png")),
                            style: Style {
                                width: Val::Px(32.),
                                height: Val::Px(32.),
                                ..default()
                            },
                            ..default()
                        },
                        TextureAtlas {
                            layout: atlas_layout.handle.clone(),
                            index: 4,
                        },
                    ));
                    parent.spawn((
                        HudText {
                            id: HudTextId::high_score,
                        },
                        TextBundle {
                            text: Text::from_sections([TextSection {
                                value: " 0".into(),
                                style: TextStyle {
                                    font_size: 32.0,
                                    color: Color::BLACK,
                                    ..default()
                                },
                            }]),
                            ..default()
                        },
                    ));
                });

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
                    justify_content: JustifyContent::SpaceAround,
                    width: Val::Percent(100.),
                    height: Val::Px(100.),
                    top: Val::Px(500.0),
                    // give it some padding for readability
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
                            width: Val::Percent(20.),
                            align_content: AlignContent::Center,
                            justify_content: JustifyContent::Center,
                            ..default()
                        },
                        ..default()
                    },
                ))
                .with_children(|parent| {
                    parent.spawn((
                        MenuText,
                        ImageBundle {
                            transform: Transform::from_rotation(Quat::from_rotation_z(
                                -std::f32::consts::FRAC_PI_2,
                            )), // -90-degree rotation in radians
                            style: Style {
                                flex_shrink: 1.0,
                                ..default()
                            },
                            image: UiImage::new(asset_server.load("images\\fairy-button.png")),
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
                            width: Val::Percent(20.),
                            align_content: AlignContent::Center,
                            justify_content: JustifyContent::Center,
                            ..default()
                        },
                        ..default()
                    },
                ))
                .with_children(|parent| {
                    parent.spawn((
                        MenuText,
                        ImageBundle {
                            style: Style {
                                flex_shrink: 1.0,
                                ..default()
                            },
                            image: UiImage::new(asset_server.load("images\\fairy-button.png")),
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
                            width: Val::Percent(20.),
                            align_content: AlignContent::Center,
                            justify_content: JustifyContent::Center,
                            ..default()
                        },
                        ..default()
                    },
                ))
                .with_children(|parent| {
                    parent.spawn((
                        MenuText,
                        ImageBundle {
                            transform: Transform::from_rotation(Quat::from_rotation_z(
                                std::f32::consts::FRAC_PI_2 * 2.,
                            )), // -90-degree rotation in radians
                            style: Style {
                                flex_shrink: 1.0,
                                ..default()
                            },
                            image: UiImage::new(asset_server.load("images\\fairy-button.png")),
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
                            width: Val::Percent(20.),
                            align_content: AlignContent::Center,
                            justify_content: JustifyContent::Center,
                            ..default()
                        },
                        ..default()
                    },
                ))
                .with_children(|parent| {
                    parent.spawn((
                        MenuText,
                        ImageBundle {
                            transform: Transform::from_rotation(Quat::from_rotation_z(
                                std::f32::consts::FRAC_PI_2,
                            )), // 90-degree rotation in radians
                            style: Style {
                                flex_shrink: 1.0,
                                ..default()
                            },
                            image: UiImage::new(asset_server.load("images\\fairy-button.png")),
                            ..default()
                        },
                    ));
                });
        });
}
