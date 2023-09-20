use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use prelude::train_plugin::*;

use crate::prelude::{camera_plugin::CameraPlugin, train::{create_train, create_train_test, move_train, list_trains}};

mod prelude;



fn main() {
    println!("Starting.");


    App::new()
        .add_plugins((DefaultPlugins, CameraPlugin, TrainPlugin))
        .add_systems(Startup, create_train_test)
        .add_systems(Update, (move_train, list_trains))
        .run();


    println!("Ending.");
}
