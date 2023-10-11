use bevy::prelude::*;

use crate::player::PlayerComponent;

const GHOST_SPEED: f32 = 3.0;
const GHOST_SCALE: f32 = 4.0;

#[derive(Component)]
pub struct GhostComponent {
    pub attack_behavior: AttackBehaviorType,
    pub speed: f32,
}

pub enum AttackBehaviorType {
    DirectPursuit,
}

pub fn spawn_ghosts_system(mut commands: Commands, asset_server: Res<AssetServer>) {
    // spawn red ghost
    commands
        .spawn(GhostComponent {
            attack_behavior: AttackBehaviorType::DirectPursuit,
            speed: GHOST_SPEED,
        })
        .insert(SpriteBundle {
            texture: asset_server.load("red_ghost.png"),
            transform: Transform::from_scale(Vec3::splat(GHOST_SCALE))
                .with_translation(Vec3::new(500.0, 0.0, 0.0)),
            ..default()
        });
}

pub fn ghost_attack_system(
    mut ghost_query: Query<(&mut Transform, &GhostComponent), Without<PlayerComponent>>,
    player_query: Query<&Transform, With<PlayerComponent>>,
) {
    if let Ok(player_transform) = player_query.get_single() {
        for (mut ghost_transform, ghost_component) in ghost_query.iter_mut() {
            match ghost_component.attack_behavior {
                AttackBehaviorType::DirectPursuit => {
                    let delta_position = (player_transform.translation
                        - ghost_transform.translation)
                        .normalize_or_zero()
                        * ghost_component.speed;

                    ghost_transform.translation += delta_position;
                }
            }
        }
    } else {
        error!("No player entity found.")
    }
}
