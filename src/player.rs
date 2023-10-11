use bevy::prelude::*;

#[derive(Component)]
struct PlayerComponent;

pub fn spawn_player_system(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(PlayerComponent).insert(SpriteBundle {
        texture: asset_server.load("pacman.png"),
        transform: Transform::from_scale(Vec3::splat(4.0)),
        ..default()
    });
}
