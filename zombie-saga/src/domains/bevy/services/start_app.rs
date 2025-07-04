use bevy::prelude::*;
use bevy::window::WindowResolution;

pub fn start_app() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        resizable: true,
                        canvas: Some("#window-jumper".into()),
                        resolution: WindowResolution::new(400., 600.),
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        // .add_plugins((GraphicsPlugin, GamePlugin, MenuPlugin))
        .run();
}
