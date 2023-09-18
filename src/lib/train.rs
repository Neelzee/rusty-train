use bevy::prelude::*;

#[derive(Component)]
pub struct Train;


#[derive(Component)]
pub struct Name(String);

#[derive(Resource)]
pub struct TrainTimer(pub Timer);

pub fn add_trains(mut commands: Commands) {
    commands.spawn((Train, Name("Thomas Toget".to_string())));
    commands.spawn((Train, Name("Nils Toget".to_string())));
    commands.spawn((Train, Name("Mikal Toget".to_string())));
}


pub fn list_trains(query: Query<&Name, With<Train>>, time: Res<Time>, mut timer: ResMut<TrainTimer>) {

    if timer.0.tick(time.delta()).just_finished() {
        for name in &query {
            println!("Train: {}", name.0);
        }
    }

}