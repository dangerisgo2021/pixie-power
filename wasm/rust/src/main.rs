#![allow(clippy::type_complexity)]
#![allow(clippy::too_many_arguments)]

use crate::domains::bevy::services::start_app::start_app;

mod domains;
fn main() {
    println!("Hello, pixie power!");
    start_app();
}

