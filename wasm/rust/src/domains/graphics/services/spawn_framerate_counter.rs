use crate::domains::graphics::domain_objects::framerate_counter::{
    FramerateCounter, FramerateText,
};
use bevy::prelude::*;

pub fn spawn_framerate_counter(mut commands: Commands) {
    commands
        .spawn((
            FramerateCounter,
            NodeBundle {
                background_color: BackgroundColor(Color::BLACK),
                // make it "always on top" by setting the Z index to maximum
                // we want it to be displayed over all other UI
                z_index: ZIndex::Global(i32::MAX),
                style: Style {
                    position_type: PositionType::Absolute,
                    // position it in the top-right corner
                    // 1% away from the top window edge
                    right: Val::Percent(1.),
                    top: Val::Percent(1.),
                    // set bottom/left to Auto, so it can be
                    // automatically sized depending on the text
                    bottom: Val::Auto,
                    left: Val::Auto,
                    // give it some padding for readability
                    padding: UiRect::all(Val::Px(4.0)),
                    ..Default::default()
                },
                ..default()
            },
        ))
        .with_children(|parent| {
            parent.spawn((
                FramerateText,
                TextBundle {
                    text: Text::from_sections([
                        TextSection {
                            value: "FPS: ".into(),
                            style: TextStyle {
                                font_size: 16.0,
                                color: Color::WHITE,
                                // if you want to use your game's font asset,
                                // uncomment this and provide the handle:
                                // font: my_font_handle
                                ..default()
                            },
                        },
                        TextSection {
                            value: " N/A".into(),
                            style: TextStyle {
                                font_size: 16.0,
                                color: Color::WHITE,
                                // if you want to use your game's font asset,
                                // uncomment this and provide the handle:
                                // font: my_font_handle
                                ..default()
                            },
                        },
                    ]),
                    ..default()
                },
            ));
        });
}
