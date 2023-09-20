use bevy::prelude::*;

use super::train::{TrainTimer, add_trains, find_train_by_id};



pub struct TrainPlugin;


impl Plugin for TrainPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(TrainTimer(Timer::from_seconds(2.0, TimerMode::Repeating)))
            .add_systems(Startup, add_trains);
            //.add_systems(Update, list_trains);
            
    }
}


pub struct DebugTrain;

impl Plugin for DebugTrain {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, find_train_by_id);
    }
}