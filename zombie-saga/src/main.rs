#![allow(clippy::type_complexity)]
#![allow(clippy::too_many_arguments)]

mod domains;

use crate::domains::bevy::services::start_app::start_app;

fn main() {
    println!("starting zombie saga!");
    start_app();
}

