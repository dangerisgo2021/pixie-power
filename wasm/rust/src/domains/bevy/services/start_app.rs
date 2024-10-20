// use std::io;
// use std::io::BufRead;
// use std::time::Duration;
use crate::domains::game::plugins::game_plugin::GamePlugin;
use crate::domains::graphics::plugins::graphics_plugin::GraphicsPlugin;
use crate::domains::menus::plugins::menu_plugin::MenuPlugin;

use bevy::{prelude::*};

pub fn start_app() {
    App::new()
        // .add_plugins(MinimalPlugins)
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins((GraphicsPlugin, GamePlugin, MenuPlugin))
        // .insert_resource(Time::<Virtual>::from_max_delta(Duration::from_secs(5)))
        // .insert_resource(Time::<Fixed>::from_duration(Duration::from_secs(1)))
        // .add_systems(PreUpdate, print_real_time)
        // .add_systems(FixedUpdate, print_fixed_time)
        // .add_systems(Update, print_time)
        // .set_runner(runner)
        .run();
}

// fn banner() {
//     println!("This example is meant to intuitively demonstrate how Time works in Bevy.");
//     println!();
//     println!("Time will be printed in three different schedules in the app:");
//     println!("- PreUpdate: real time is printed");
//     println!("- FixedUpdate: fixed time step time is printed, may be run zero or multiple times");
//     println!("- Update: virtual game time is printed");
//     println!();
//     println!("Max delta time is set to 5 seconds. Fixed timestep is set to 1 second.");
//     println!();
// }

// fn help() {
//     println!("The app reads commands line-by-line from standard input.");
//     println!();
//     println!("Commands:");
//     println!("  empty line: Run app.update() once on the Bevy App");
//     println!("  q: Quit the app.");
//     println!("  f: Set speed to fast, 2x");
//     println!("  n: Set speed to normal, 1x");
//     println!("  s: Set speed to slow, 0.5x");
//     println!("  p: Pause");
//     println!("  u: Unpause");
// }

// fn runner(mut app: App) -> AppExit {
//     banner();
//     help();
//     let stdin = io::stdin();
//     for line in stdin.lock().lines() {
//         if let Err(err) = line {
//             println!("read err: {:#}", err);
//             break;
//         }
//         match line.unwrap().as_str() {
//             "" => {
//                 app.update();
//             }
//             "f" => {
//                 println!("FAST: setting relative speed to 2x");
//                 app.world_mut()
//                     .resource_mut::<Time<Virtual>>()
//                     .set_relative_speed(2.0);
//             }
//             "n" => {
//                 println!("NORMAL: setting relative speed to 1x");
//                 app.world_mut()
//                     .resource_mut::<Time<Virtual>>()
//                     .set_relative_speed(1.0);
//             }
//             "s" => {
//                 println!("SLOW: setting relative speed to 0.5x");
//                 app.world_mut()
//                     .resource_mut::<Time<Virtual>>()
//                     .set_relative_speed(0.5);
//             }
//             "p" => {
//                 println!("PAUSE: pausing virtual clock");
//                 app.world_mut().resource_mut::<Time<Virtual>>().pause();
//             }
//             "u" => {
//                 println!("UNPAUSE: resuming virtual clock");
//                 app.world_mut().resource_mut::<Time<Virtual>>().unpause();
//             }
//             "q" => {
//                 println!("QUITTING!");
//                 break;
//             }
//             _ => {
//                 help();
//             }
//         }
//     }
// 
//     AppExit::Success
// }
// 
// fn print_real_time(time: Res<Time<Real>>) {
//     println!(
//         "PreUpdate: this is real time clock, delta is {:?} and elapsed is {:?}",
//         time.delta(),
//         time.elapsed()
//     );
// }
// 
// fn print_fixed_time(time: Res<Time>) {
//     println!(
//         "FixedUpdate: this is generic time clock inside fixed, delta is {:?} and elapsed is {:?}",
//         time.delta(),
//         time.elapsed()
//     );
// }
// 
// fn print_time(time: Res<Time>) {
//     println!(
//         "Update: this is generic time clock, delta is {:?} and elapsed is {:?}",
//         time.delta(),
//         time.elapsed()
//     );
// }