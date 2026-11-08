use bevy::prelude::*;

use crate::state::GameState;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct PhysicsSystems;

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            apply_velocity
                .in_set(PhysicsSystems)
                .run_if(in_state(GameState::Playing)),
        );
    }
}

/// Movement vector in world units per second.
/// Automatically ensures the entity also has a Transform.
#[derive(Component, Default, Clone, Copy, Debug, Deref, DerefMut)]
#[require(Transform)]
pub struct Velocity(pub Vec2);

/// Axis-aligned 2D hitbox storing half-extents (width / 2.0, height / 2.0).
#[derive(Component, Default, Clone, Copy, Debug)]
#[require(Transform)]
pub struct Collider {
    pub half_size: Vec2,
}

impl Collider {
    pub fn new(width: f32, height: f32) -> Self {
        Self {
            half_size: Vec2::new(width * 0.5, height * 0.5),
        }
    }

    /// Builds a Bevy Aabb2d at the entity's current world position
    pub fn aabb(&self, translation: Vec3) -> Aabb2d {
        Aabb2d::new(translation.truncate(), self.half_size)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CollisionSide {
    Left,
    Right,
    Top,
    Bottom,
}

/// Checks if `moving_aabb` intersects `target_aabb` and returns which side of `target_aabb` was hit
pub fn check_aabb_collision(moving_aabb: Aabb2d, target_aabb: Aabb2d) -> Option<CollisionSide> {
    if !moving_aabb.intersects(&target_aabb) {
        return None;
    }

    // Closest point on the target's box to the moving object's center
    let closest = target_aabb.closest_point(moving_aabb.center());
    let offset = moving_aabb.center() - closest;

    if offset.x.abs() > offset.y.abs() {
        if offset.x < 0.0 {
            Some(CollisionSide::Left)
        } else {
            Some(CollisionSide::Right)
        }
    } else if offset.y > 0.0 {
        Some(CollisionSide::Top)
    } else {
        Some(CollisionSide::Bottom)
    }
}

/// Moves every entity with a Velocity component each frame
fn apply_velocity(time: Res<Time>, mut query: Query<(&Velocity, &mut Transform)>) {
    let dt = time.delta_secs();
    for (velocity, mut transform) in &mut query {
        transform.translation += velocity.extend(0.0) * dt;
    }
}
