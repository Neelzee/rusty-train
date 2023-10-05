//! Renders a 2D scene containing a single, moving sprite.

mod prelude;

use bevy::prelude::*;
use prelude::train::Direction;


fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, sprite_movement)
        .run();
}


fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("debug.png"),
            transform: Transform::from_xyz(100., 0., 0.),
            ..default()
        },
        Direction::North,
    ));
}

/// The sprite is animated by changing its translation depending on the time that has passed since
/// the last frame.
fn sprite_movement(time: Res<Time>, mut sprite_position: Query<(&mut Direction, &mut Transform)>) {
    for (mut logo, mut transform) in &mut sprite_position {
        match *logo {
            Direction::North => transform.translation.y += 150. * time.delta_seconds(),
            Direction::South => transform.translation.y -= 150. * time.delta_seconds(),
            _ => {}
        }

        if transform.translation.y > 200. {
            *logo = Direction::South;
        } else if transform.translation.y < -200. {
            *logo = Direction::North;
        }
    }
}

