use bevy::prelude::*;

use crate::player::PlayerComponent;

// Define constants for ghost attributes
const GHOST_SPEED: f32 = 3.0; // Speed at which the ghost moves
const GHOST_SCALE: f32 = 4.0; // Scale factor for the ghost sprite

// Define a GhostComponent struct to represent ghost entities in the game
#[derive(Component)]
pub struct GhostComponent {
    pub attack_behavior: AttackBehaviorType, // Behavior type when attacking the player
    pub speed: f32,                          // Speed of the ghost
}

// Enum to define different types of attack behaviors for ghosts
pub enum AttackBehaviorType {
    DirectPursuit, // Ghost directly pursues the player
    ShyPursuit,              
    UpandDown(UpandDownWithBreakData),
   // Add new ghost behaviors here!
}

pub struct UpandDownWithBreakData {
    pub timer: Timer,
    pub y_velocity: f32,
}


impl UpandDownWithBreakData {
    pub fn new(y_velocity: f32) -> Self {
        UpandDownWithBreakData {
            y_velocity: y_velocity,
            timer: Timer::from_seconds(2.0, TimerMode::Repeating),
        }

    }
}

// System function to spawn ghost entities in the game
pub fn spawn_ghosts_system(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Spawn a red ghost with DirectPursuit behavior
    commands
        .spawn(GhostComponent {
            attack_behavior: AttackBehaviorType::DirectPursuit,
            speed: GHOST_SPEED,
        })
        .insert(SpriteBundle {
            texture: asset_server.load("red_ghost.png"), // Load the red ghost texture
            transform: Transform::from_scale(Vec3::splat(GHOST_SCALE))
                .with_translation(Vec3::new(500.0, 0.0, 0.0)), // Set the scale and initial position of the ghost
            ..default()
        });
        commands
        .spawn(GhostComponent {
            attack_behavior: AttackBehaviorType::ShyPursuit,
            speed: GHOST_SPEED,
        })
        .insert(SpriteBundle {
            texture: asset_server.load("brown_ghost.png"), // Load the red ghost texture
            transform: Transform::from_scale(Vec3::splat(GHOST_SCALE))
                .with_translation(Vec3::new(500.0, 0.0, 0.0)), // Set the scale and initial position of the ghost
            ..default()
        });

        commands
        .spawn(GhostComponent {
            attack_behavior: AttackBehaviorType::UpandDown(
                UpandDownWithBreakData::new(2.0),
            ),
            speed: GHOST_SPEED,
        })
        .insert(SpriteBundle {
            texture: asset_server.load("pink_ghost.png"), // Load the red ghost texture
            transform: Transform::from_scale(Vec3::splat(GHOST_SCALE))
                .with_translation(Vec3::new(0.0, 500.0, 0.0)), // Set the scale and initial position of the ghost
            ..default()
        });
        


    // Spawn additional ghosts here!
}

// System function to handle the attack behavior of ghosts
pub fn ghost_attack_system(
    mut ghost_query: Query<(&mut Transform, &mut GhostComponent), Without<PlayerComponent>>, // Query ghosts without the PlayerComponent
    player_query: Query<&Transform, With<PlayerComponent>>, // Query the player's transform
    time: Res<Time>,
) {
    // Check if a player entity exists
    if let Ok(player_transform) = player_query.get_single() {
        // Iterate over all ghost entities
        for (mut ghost_transform, mut ghost_component) in ghost_query.iter_mut() {
            // Match ghost behaviors here!
            match &mut ghost_component.attack_behavior {
                // If the ghost's attack behavior is DirectPursuit
                AttackBehaviorType::DirectPursuit => {
                    // Calculate the direction and distance to move towards the player
                    if (player_transform.translation.distance_squared(ghost_transform.translation)) > 10.0 {
                        let delta_position = (player_transform.translation
                        - ghost_transform.translation)
                        .normalize_or_zero()
                        * ghost_component.speed;

                    // Update the ghost's position
                    ghost_transform.translation += delta_position;
                    }
                    
                },
                AttackBehaviorType::UpandDown(data) => {
                    data.timer.tick(time.delta());

                    if data.timer.just_finished() {
                        data.y_velocity *= -1.0;
                    }

                    ghost_transform.translation.y -= data.y_velocity;
                }
                AttackBehaviorType::ShyPursuit => {
                    let mut delta_position: Vec3;
                    let distance = player_transform.translation.distance_squared(ghost_transform.translation);
                    if distance < 40000.0 {
                        delta_position = -(player_transform.translation - ghost_transform.translation)
                        .normalize_or_zero() * ghost_component.speed;
                    } else if distance > 44100.0 {
                        delta_position = (player_transform.translation - ghost_transform.translation)
                        .normalize_or_zero() * ghost_component.speed;
                    }
                    else {
                        delta_position = Vec3::new(0.0,0.0,0.0);
                    }
                    ghost_transform.translation += delta_position;
                    
                }
            }
        }
    } else {
        // Log an error if no player entity is found
        error!("No player entity found.")
    }
}
