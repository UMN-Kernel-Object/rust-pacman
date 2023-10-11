use bevy::prelude::*;

mod player;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, player::spawn_player_system)
        .run();
}
