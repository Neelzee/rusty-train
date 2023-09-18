use bevy::prelude::*;

#[derive(Component)]
pub struct Train;


#[derive(Component)]
pub struct Id(u128);


#[derive(Component)]
pub struct Name(String);

#[derive(Resource)]
pub struct TrainTimer(pub Timer);

#[derive(Component)]
pub struct Position(f32, f32);

pub fn add_trains(mut cmd: Commands) {
    cmd.spawn((
        Train, 
        Id(0), 
        Name("Nils".to_string()), 
        Position(0f32, 0f32))
    );

    cmd.spawn((
        Train, 
        Id(1), 
        Name("Thomas".to_string()), 
        Position(0f32, 0f32))
    );

    cmd.spawn((
        Train, 
        Id(2), 
        Name("Mikal".to_string()), 
        Position(0f32, 0f32))
    );
}


pub fn create_train(mut cmd: Commands, name: String, id: u128) {
    cmd.spawn((
        Train, 
        Id(id), 
        Name(name), 
        Position(0f32, 0f32))
    );
}


pub fn find_train_by_id(query: Query<(&Id, &Name, &Position), With<Train>>) {
    for (id, nm, ps) in &query {
        if id.0 == 0 {
            println!("Train: {} at position {}.{}", nm.0, ps.0, ps.1);
        } else {
            println!("Not valid train:\nId: {} Train: {} at position {}.{}\n", id.0, nm.0, ps.0, ps.1);
        }
    }
}


pub fn list_trains(query: Query<&Name, With<Train>>, time: Res<Time>, mut timer: ResMut<TrainTimer>) {

    if timer.0.tick(time.delta()).just_finished() {
        for name in &query {
            println!("Train: {}", name.0);
        }
    }

}