use bevy::prelude::*;
pub fn _exit_system(mut exit: EventWriter<AppExit>) {
    exit.send(AppExit::Success);
} 