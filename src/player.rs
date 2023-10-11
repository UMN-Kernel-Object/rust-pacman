use bevy::prelude::*;

#[derive(Component)]
pub struct PlayerComponent;

pub fn spawn_player_system(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(PlayerComponent).insert(SpriteBundle {
        texture: asset_server.load("pacman.png"),
        transform: Transform::from_scale(Vec3::splat(4.0)),
        ..default()
    });
}

pub fn move_player_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<&mut Transform, With<PlayerComponent>>,
) {
    let pressed_inputs = keyboard_input.get_pressed();

    let mut delta_x = 0.0;
    let mut delta_y = 0.0;

    for input in pressed_inputs {
        match input {
            KeyCode::Up => delta_y += 1.0,
            KeyCode::Down => delta_y -= 1.0,
            KeyCode::Left => delta_x -= 1.0,
            KeyCode::Right => delta_x += 1.0,
            _ => {}
        }
    }

    let delta_position = Vec3::new(delta_x, delta_y, 0.0);

    if let Ok(mut player_transform) = player_query.get_single_mut() {
        player_transform.translation += delta_position;
    } else {
        error!("No player entity found.")
    }
}
