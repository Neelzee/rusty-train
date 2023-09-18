use bevy::prelude::*;

use lib::train_plugin::*;

mod lib;


fn hello_game() {
    println!("Hello, Game!");
}


fn main() {
    println!("Starting.");


    App::new()
        .add_plugins((DefaultPlugins, TrainPlugin, DebugTrain))
        .run();


    println!("Ending.");
}
