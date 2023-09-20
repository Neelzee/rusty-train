use bevy::prelude::*;

#[derive(Component)]
pub struct CameraPlugin;


impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, camera_setup);
    }
}


fn camera_setup(mut cmd: Commands) {
    cmd.spawn(
        Camera2dBundle::default()
    );
}