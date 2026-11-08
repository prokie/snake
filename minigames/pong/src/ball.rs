// minigames/pong/src/ball.rs
use bevy::prelude::*;
use lightyear::prelude::{server::*, *};
use protocol::{BallMarker, PaddleSide, PlayerSide, Position2D};

use crate::constants::*;
use crate::paddle::PaddleSystems;
use crate::physics::{Collider, CollisionSide, Velocity, check_aabb_collision};
use crate::state::GameState;

pub struct BallPlugin;

impl Plugin for BallPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), server_spawn_ball)
            .add_systems(
                FixedUpdate,
                (bounce_off_walls, collide_with_paddles, check_goal_scored)
                    .chain()
                    .after(PaddleSystems)
                    .run_if(in_state(GameState::Playing)),
            )
            .add_observer(on_point_scored_reset_ball)
            .add_observer(attach_ball_visuals);
    }
}

#[derive(Event, Clone, Copy, Debug)]
pub struct PointScored {
    pub scorer: PlayerSide,
}

fn server_spawn_ball(mut commands: Commands, server: Query<(), With<Endpoint>>) {
    if server.is_empty() {
        return;
    }

    commands.spawn((
        DespawnOnExit(GameState::Playing),
        BallMarker,
        Position2D(Vec2::ZERO),
        Transform::from_xyz(0.0, 0.0, 1.0),
        Velocity(
            Vec2::new(INITIAL_BALL_SPEED, INITIAL_BALL_SPEED * 0.4).normalize()
                * INITIAL_BALL_SPEED,
        ),
        Collider::new(BALL_SIZE, BALL_SIZE),
        Replicate::to_clients(NetworkTarget::All),
        InterpolationTarget::to_clients(NetworkTarget::All),
    ));
}

fn attach_ball_visuals(trigger: On<Add<BallMarker>>, mut commands: Commands) {
    commands.entity(trigger.entity).insert((
        DespawnOnExit(GameState::Playing),
        Sprite {
            color: Color::srgb(1.0, 0.85, 0.2),
            custom_size: Some(Vec2::splat(BALL_SIZE)),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 1.0),
    ));
}

fn bounce_off_walls(
    mut ball_query: Query<(&mut Position2D, &mut Velocity, &Collider), With<BallMarker>>,
) {
    let Ok((mut pos, mut velocity, collider)) = ball_query.single_mut() else {
        return;
    };
    let max_y = (ARENA_HEIGHT * 0.5) - collider.half_size.y;

    if pos.y > max_y && velocity.y > 0.0 {
        pos.y = max_y;
        velocity.y = -velocity.y;
    } else if pos.y < -max_y && velocity.y < 0.0 {
        pos.y = -max_y;
        velocity.y = -velocity.y;
    }
}

fn collide_with_paddles(
    mut ball_query: Query<(&Position2D, &mut Velocity, &Collider), With<BallMarker>>,
    paddle_query: Query<
        (&Position2D, &Collider),
        (With<PaddleSide>, Without<BallMarker>, Without<Interpolated>),
    >,
) {
    let Ok((ball_pos, mut ball_vel, ball_collider)) = ball_query.single_mut() else {
        return;
    };
    let ball_aabb = ball_collider.aabb(ball_pos.0);

    for (paddle_pos, paddle_collider) in &paddle_query {
        let paddle_aabb = paddle_collider.aabb(paddle_pos.0);

        let Some(side) = check_aabb_collision(ball_aabb, paddle_aabb) else {
            continue;
        };

        match side {
            CollisionSide::Left if ball_vel.x > 0.0 => {
                apply_paddle_bounce(&mut ball_vel, ball_pos.y, paddle_pos.y, -1.0);
            }
            CollisionSide::Right if ball_vel.x < 0.0 => {
                apply_paddle_bounce(&mut ball_vel, ball_pos.y, paddle_pos.y, 1.0);
            }
            CollisionSide::Top if ball_vel.y < 0.0 => ball_vel.y = -ball_vel.y,
            CollisionSide::Bottom if ball_vel.y > 0.0 => ball_vel.y = -ball_vel.y,
            _ => {}
        }
    }
}

fn apply_paddle_bounce(ball_vel: &mut Vec2, ball_y: f32, paddle_y: f32, new_x_sign: f32) {
    let speed = (ball_vel.length() * BALL_SPEED_INCREMENT).min(1200.0);
    let hit_factor = ((ball_y - paddle_y) / (PADDLE_HEIGHT * 0.5)).clamp(-1.0, 1.0);
    let bounce_dir = Vec2::new(new_x_sign, hit_factor * 0.75).normalize();
    *ball_vel = bounce_dir * speed;
}

fn check_goal_scored(
    mut commands: Commands,
    ball_query: Query<&Position2D, (With<BallMarker>, Without<Interpolated>)>,
) {
    let Ok(pos) = ball_query.single() else {
        return;
    };
    let goal_x = ARENA_WIDTH * 0.5;

    if pos.x > goal_x {
        commands.trigger(PointScored {
            scorer: PlayerSide::Left,
        });
    } else if pos.x < -goal_x {
        commands.trigger(PointScored {
            scorer: PlayerSide::Right,
        });
    }
}

fn on_point_scored_reset_ball(
    point: On<PointScored>,
    mut ball_query: Query<
        (&mut Position2D, &mut Velocity),
        (With<BallMarker>, Without<Interpolated>),
    >,
) {
    let Ok((mut pos, mut velocity)) = ball_query.single_mut() else {
        return;
    };
    pos.0 = Vec2::ZERO;

    let serve_x = match point.scorer {
        PlayerSide::Left => 1.0,
        PlayerSide::Right => -1.0,
    };
    **velocity = Vec2::new(serve_x, 0.35).normalize() * INITIAL_BALL_SPEED;
}
