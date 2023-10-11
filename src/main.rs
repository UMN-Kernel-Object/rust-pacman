use bevy::prelude::*;

mod camera;
mod player;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(
            // This sets image filtering to nearest
            // This is done to prevent textures with low resolution (e.g. pixel art) from being blurred
            // by linear filtering.
            ImagePlugin::default_nearest(),
        ))
        .add_systems(
            Startup,
            (camera::spawn_camera_system, player::spawn_player_system),
        )
        .add_systems(Update, player::move_player_system)
        .run();
}
