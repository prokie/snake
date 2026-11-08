use crate::state::GameState;
use bevy::prelude::*;
use lightyear::prelude::{Interpolated, Predicted};
use protocol::Position2D;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct PhysicsSystems;

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedUpdate,
            apply_velocity
                .in_set(PhysicsSystems)
                .run_if(in_state(GameState::Playing)),
        )
        .add_systems(
            PostUpdate,
            sync_position_to_transform.run_if(in_state(GameState::Playing)),
        );
    }
}

#[derive(Component, Default, Clone, Copy, Debug, Deref, DerefMut)]
#[require(Position2D, Transform)]
pub struct Velocity(pub Vec2);

#[derive(Component, Default, Clone, Copy, Debug)]
#[require(Position2D, Transform)]
pub struct Collider {
    pub half_size: Vec2,
}

impl Collider {
    pub fn new(width: f32, height: f32) -> Self {
        Self {
            half_size: Vec2::new(width * 0.5, height * 0.5),
        }
    }

    pub fn aabb(&self, pos: Vec2) -> Aabb2d {
        Aabb2d::new(pos, self.half_size)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CollisionSide {
    Left,
    Right,
    Top,
    Bottom,
}

pub fn check_aabb_collision(moving_aabb: Aabb2d, target_aabb: Aabb2d) -> Option<CollisionSide> {
    if !moving_aabb.intersects(&target_aabb) {
        return None;
    }
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

fn apply_velocity(time: Res<Time>, mut query: Query<(&Velocity, &mut Position2D)>) {
    let dt = time.delta_secs();
    for (velocity, mut pos) in &mut query {
        if velocity.0 != Vec2::ZERO {
            pos.0 += velocity.0 * dt;
        }
    }
}

/// Copies networked/predicted/interpolated Position2D into Bevy's Transform for rendering
fn sync_position_to_transform(
    mut query: Query<
        (&Position2D, &mut Transform),
        Or<(With<Predicted>, With<Interpolated>, With<Collider>)>,
    >,
) {
    for (pos, mut transform) in &mut query {
        transform.translation.x = pos.x;
        transform.translation.y = pos.y;
    }
}
