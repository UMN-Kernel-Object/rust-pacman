use bevy::prelude::*;

// Define constants for player attributes
const PLAYER_SPEED: f32 = 5.0; // Speed at which the player moves
const PLAYER_SCALE: f32 = 4.0; // Scale factor for the player sprite

// Define a PlayerComponent struct to represent the player entity in the game
#[derive(Component)]
pub struct PlayerComponent {
    pub speed: f32, // Speed of the player
}

// System function to spawn the player entity in the game
pub fn spawn_player_system(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn(PlayerComponent {
            speed: PLAYER_SPEED,
        })
        .insert(SpriteBundle {
            texture: asset_server.load("pacman.png"), // Load the player texture
            transform: Transform::from_scale(Vec3::splat(PLAYER_SCALE)), // Set the scale of the player sprite
            ..default()
        });
}

// System function to handle player movement based on keyboard input
pub fn move_player_system(
    keyboard_input: Res<Input<KeyCode>>, // Capture keyboard input
    mut player_query: Query<(&mut Transform, &PlayerComponent)>, // Query the player's transform and component
) {
    // Get the set of pressed keys
    let pressed_inputs = keyboard_input.get_pressed();

    // Initialize movement deltas for x and y axes
    let mut delta_x = 0.0;
    let mut delta_y = 0.0;

    // Determine movement direction based on pressed keys
    for input in pressed_inputs {
        match input {
            KeyCode::Up => delta_y += 1.0,    // Move up
            KeyCode::Down => delta_y -= 1.0,  // Move down
            KeyCode::Left => delta_x -= 1.0,  // Move left
            KeyCode::Right => delta_x += 1.0, // Move right
            _ => {}                           // Ignore other keys
        }
    }

    // Calculate the movement vector
    let delta_position = Vec3::new(delta_x, delta_y, 0.0);

    // Update the player's position based on the movement vector and speed
    if let Ok((mut player_transform, player_component)) = player_query.get_single_mut() {
        player_transform.translation += player_component.speed * delta_position.normalize_or_zero();
    } else {
        // Log an error if no player entity is found
        error!("No player entity found.")
    }
}
