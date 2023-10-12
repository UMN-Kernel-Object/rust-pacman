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
    DirectPursuit,                                      // Ghost directly pursues the player
    DirectPursuitWithBreak(DirectPursuitWithBreakData),
    ShyPursuit,              
    UpandDown(UpandDownWithBreakData),
    CirclePursuit(CirclePursuitData),
  // Add new ghost behaviors here!
}


pub struct CirclePursuitData {
    // angle: where around the player the ghost is
    pub angle: f32,
    // how far away the ghost starts from the player
    pub base_radius: f32,
    // used for the sinusoidal movement, how far away the peaks of the wave
    // are from the base radius
    pub radius_delta: f32,
    // how much to modify the angle when calculating the radius at timestep
    pub angle_rad_mod: f32,
    // functions to better control ghosts moving in different circle patterns
    pub x_angle_func: fn(f32)->f32,
    pub y_angle_func: fn(f32)->f32,
    pub rad_angle_func: fn(f32)->f32,
}

impl CirclePursuitData {
    pub fn new(
        a: f32, br: f32, rd: f32, arm: f32, 
        xaf: fn(f32)->f32, yaf: fn(f32)->f32, raf: fn(f32)->f32
    ) -> Self { 
        Self { 
            angle: a, base_radius: br, 
            radius_delta: rd, angle_rad_mod: arm, 
            x_angle_func: xaf, y_angle_func: yaf, 
            rad_angle_func: raf 
        } 
    }

    pub fn update_angle(&mut self, dt: f32) {
        let two_pi = std::f32::consts::PI * 2.0;
        let flag: f32 = (self.angle >= two_pi).into();
        // cap data.angle into range between 0 and 2pi
        // don't want data.angle to get too big and lose precision
        self.angle -= flag * two_pi;
        self.angle += GHOST_SPEED * dt;
    }

    pub fn update_transform(&self, pt: &Vec3, gt: &mut Vec3) {
        // get radius using given function
        let radius = self.base_radius + (
            (self.rad_angle_func)(self.angle*self.angle_rad_mod) * 
            self.radius_delta
        );

        // update position using given x and y funcs and computer radius
        gt.x = pt.x + (self.x_angle_func)(self.angle) * radius;
        gt.y = pt.y + (self.y_angle_func)(self.angle) * radius;
    }
}

pub struct DirectPursuitWithBreakData {
    pub timer: Timer,
    pub rest: bool,
}

impl DirectPursuitWithBreakData {
    pub fn new(rest_time: f32) -> Self {
        DirectPursuitWithBreakData {
            timer: Timer::from_seconds(rest_time, TimerMode::Repeating),
            rest: true,
        }
    }
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
            attack_behavior: AttackBehaviorType::DirectPursuitWithBreak(
                DirectPursuitWithBreakData::new(1.5),
            ),
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
    commands
        .spawn(GhostComponent {
            attack_behavior: AttackBehaviorType::CirclePursuit(
                CirclePursuitData::new(
                    0.0, 200.0, 50.0, 3.0,
                    f32::cos, f32::sin, f32::sin
                ),
            ),
            speed: GHOST_SPEED,
        })
        .insert(SpriteBundle {
            texture: asset_server.load("purple_ghost.png"), // Load the red ghost texture
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
                    let delta_position = (player_transform.translation
                        - ghost_transform.translation)
                        .normalize_or_zero()
                        * ghost_component.speed;

                    // Update the ghost's position
                    ghost_transform.translation += delta_position;
                },
                AttackBehaviorType::UpandDown(data) => {
                    data.timer.tick(time.delta());

                    if data.timer.just_finished() {
                        data.y_velocity *= -1.0;
                    }

                    ghost_transform.translation.y -= data.y_velocity;
                },
                AttackBehaviorType::ShyPursuit => {
                    let mut delta_position: Vec3;
                    if player_transform.translation.distance(ghost_transform.translation) < 250.0 {
                        delta_position = -(player_transform.translation - ghost_transform.translation)
                        .normalize_or_zero() * ghost_component.speed;
                    }
                    else {
                        delta_position = (player_transform.translation - ghost_transform.translation)
                        .normalize_or_zero() * ghost_component.speed;
                    }
                    ghost_transform.translation += delta_position;
                    
                },

                // If the ghost's attack behavior is DirectPursuitWithBreak
                AttackBehaviorType::DirectPursuitWithBreak(data) => {
                    data.timer.tick(time.delta());

                    if data.timer.just_finished() {
                        data.rest = !data.rest;
                    }

                    if !data.rest {
                        // Calculate the direction and distance to move towards the player
                        let delta_position = (player_transform.translation
                            - ghost_transform.translation)
                            .normalize_or_zero()
                            * ghost_component.speed;

                        // Update the ghost's position
                        ghost_transform.translation += delta_position;
                    }
                },

                AttackBehaviorType::CirclePursuit(data) => {
                    data.update_angle(time.delta_seconds());
                    data.update_transform(
                        &player_transform.translation, 
                        &mut ghost_transform.translation
                    );
                },
            }
        }
    } else {
        // Log an error if no player entity is found
        error!("No player entity found.")
    }
}
