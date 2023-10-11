use bevy::prelude::*;

// spawn camera for rendering 2d sprites
pub fn spawn_camera_system(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
