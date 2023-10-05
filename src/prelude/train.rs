use bevy::prelude::*;
use rand::prelude::*;


#[derive(Component)]
pub struct Train;


#[derive(Component)]
pub struct Id(u128);


#[derive(Component)]
pub struct Name(String);

#[derive(Resource)]
pub struct TrainTimer(pub Timer);


pub fn add_trains(mut cmd: Commands) {
    cmd.spawn(
        (
        Train, 
        Id(0), 
        Name("Nils".to_string()), 
        )
    );

    cmd.spawn(
        (
        Train, 
        Id(1), 
        Name("Thomas".to_string()), 
        )
    );

    cmd.spawn(
        (
        Train, 
        Id(2), 
        Name("Mikal".to_string()), 
        )
    );
}


pub fn create_train(mut cmd: Commands, name: String, id: u128, ass_server: Res<AssetServer>) {
    cmd.spawn(
        (
        Train, 
        Id(id),
        SpriteBundle {
            texture: ass_server.load("/assets/debug.png"),
            transform: Transform::from_xyz(100., 100., 0.),
            ..default()
        },
        Name(name), 
        Direction::North,
        )
    );
}


pub fn find_train_by_id(query: Query<(&Id, &Name), With<Train>>) {
    for (id, nm) in &query {
        if id.0 == 0 {
            println!("Train: {}", nm.0);
        } else {
            println!("Not valid train:\nId: {} Train: {}", id.0, nm.0);
        }
    }
}


#[derive(Component)]
pub enum Direction {
    None,
    North,
    South,
    East,
    West,
    NorthEast,
    NorthWest,
    SouthEast,
    SouthWest
}

pub fn list_trains(query: Query<(&Name, &Transform), With<Train>>, time: Res<Time>, mut timer: ResMut<TrainTimer>) {

    if timer.0.tick(time.delta()).just_finished() {
        for (name, transform) in &query {
            println!("Train: {}, at: {}.{}", name.0, transform.translation.x, transform.translation.y);
        }
    }

}


pub fn random_direction() -> Direction {
    let mut rng = rand::thread_rng();

    let mut dirs: Vec<Direction> = vec![
        Direction::North,
        Direction::South,
        Direction::East,
        Direction::West,
        Direction::NorthEast,
        Direction::NorthWest,
        Direction::SouthEast,
        Direction::SouthWest
    ];

    dirs.shuffle(&mut rng);

    match dirs.pop() {
        Some(d) => d,
        None => Direction::North,
    }
}



pub fn move_train(mut query: Query<(&mut Direction, &mut Transform), With<Train>>, time: Res<Time>) {

    let vertical_speed = 150.;
    let horizontal_speed = 150.;
    let diagonal_speed = 150.;

    let top = 200.;
    let bot = -200.;
    let right = 200.;
    let left = -200.;

    for (mut heading, mut transform) in &mut query {
        match *heading {
            Direction::None => continue,
            Direction::North => transform.translation.y += vertical_speed * time.delta_seconds(),
            Direction::South => transform.translation.y -= vertical_speed * time.delta_seconds(),
            Direction::East => transform.translation.x += horizontal_speed * time.delta_seconds(),
            Direction::West => transform.translation.x -= horizontal_speed * time.delta_seconds(),
            Direction::NorthEast => {
                transform.translation.y += diagonal_speed * time.delta_seconds();
                transform.translation.x += diagonal_speed * time.delta_seconds();                

            },
            Direction::NorthWest => {
                transform.translation.y += diagonal_speed * time.delta_seconds();
                transform.translation.x -= diagonal_speed * time.delta_seconds();                

            },
            Direction::SouthEast => {
                transform.translation.y -= diagonal_speed * time.delta_seconds();
                transform.translation.x += diagonal_speed * time.delta_seconds();                

            },
            Direction::SouthWest => {
                transform.translation.y -= diagonal_speed * time.delta_seconds();
                transform.translation.x -= diagonal_speed * time.delta_seconds();                

            },
        }

        if transform.translation.y > top || transform.translation.y < bot || transform.translation.x > left || transform.translation.x < right {
            *heading = random_direction()
        }

    }
    return ();
}


pub fn create_train_test(mut cmd: Commands, ass_server: Res<AssetServer>) {
    create_train(cmd, "Nils".to_string(), 0, ass_server);
}