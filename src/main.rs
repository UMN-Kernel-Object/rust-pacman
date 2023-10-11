use bevy::prelude::*;

mod camera;
mod ghost;
mod player;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(
            // This sets image filtering to nearest
            // This is done to prevent textures with low resolution (e.g. pixel art) from being blurred
            // by linear filtering.
            ImagePlugin::default_nearest(),
        ))
        // Add systems that should run once at the start of the application
        .add_systems(
            Startup,
            (
                camera::spawn_camera_system,
                player::spawn_player_system,
                ghost::spawn_ghosts_system,
            ),
        )
        // Add systems that should run every frame during the game's update loop
        .add_systems(
            Update,
            (player::move_player_system, ghost::ghost_attack_system),
        )
        .run();
}
