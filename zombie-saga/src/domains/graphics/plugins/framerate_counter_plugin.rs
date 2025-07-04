use crate::domains::graphics::domain_objects::framerate_counter::FramerateText;
use crate::domains::graphics::domain_objects::framerate_counter::FramerateCounter;
use crate::domains::graphics::services::spawn_framerate_counter::spawn_framerate_counter;
use bevy::app::{App, Plugin, Startup};
use bevy::diagnostic::DiagnosticsStore;
use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::prelude::*;

pub struct FramerateCounterPlugin;

impl Plugin for FramerateCounterPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_framerate_counter);
        app.add_systems(Update, update_framerate_counter);
        app.add_systems(Update, fps_counter_showhide);
    }
}

fn update_framerate_counter(
    diagnostics: Res<DiagnosticsStore>,
    mut framerate_text: Query<&mut Text, With<FramerateText>>,
) {
    for mut text in &mut framerate_text  {
        if let Some(value) = diagnostics
            .get(&FrameTimeDiagnosticsPlugin::FPS)
            .and_then(|fps| fps.smoothed()) {
            text.sections[1].value = format!("{value:>4.0}");
        } else {
            text.sections[1].value = " Error".into();
        }
    }
    
}

/// Toggle the FPS counter when pressing F12
fn fps_counter_showhide(
    mut q: Query<&mut Visibility, With<FramerateCounter>>,
    kbd: Res<ButtonInput<KeyCode>>,
) {
    if kbd.just_pressed(KeyCode::F12) {
        let mut vis = q.single_mut();
        *vis = match *vis {
            Visibility::Hidden => Visibility::Visible,
            _ => Visibility::Hidden,
        };
    }
}
