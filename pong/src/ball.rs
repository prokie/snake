use bevy::prelude::*;

use crate::constants::*;
use crate::paddle::{Paddle, PaddleSystems, PlayerSide};
use crate::physics::{Collider, CollisionSide, Velocity, check_aabb_collision};
use crate::state::GameState;

pub struct BallPlugin;

impl Plugin for BallPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), spawn_ball)
            .add_systems(
                Update,
                (bounce_off_walls, collide_with_paddles, check_goal_scored)
                    .chain()
                    .after(PaddleSystems)
                    .run_if(in_state(GameState::Playing)),
            )
            .add_observer(on_point_scored_reset_ball);
    }
}

#[derive(Component)]
#[require(Velocity, Collider)]
pub struct Ball;

/// Broadcast whenever the ball passes the left or right goal line
#[derive(Event, Clone, Copy, Debug)]
pub struct PointScored {
    pub scorer: PlayerSide,
}

fn spawn_ball(mut commands: Commands) {
    commands.spawn((
        DespawnOnExit(GameState::Playing),
        Sprite {
            color: Color::srgb(1.0, 0.85, 0.2),
            custom_size: Some(Vec2::splat(BALL_SIZE)),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 1.0),
        Velocity(
            Vec2::new(INITIAL_BALL_SPEED, INITIAL_BALL_SPEED * 0.4).normalize()
                * INITIAL_BALL_SPEED,
        ),
        Collider::new(BALL_SIZE, BALL_SIZE),
        Ball,
    ));
}

/// Bounces the ball off the top and bottom arena walls
fn bounce_off_walls(ball_query: Single<(&mut Transform, &mut Velocity, &Collider), With<Ball>>) {
    let (mut transform, mut velocity, collider) = ball_query.into_inner();
    let max_y = (ARENA_HEIGHT * 0.5) - collider.half_size.y;

    if transform.translation.y > max_y && velocity.y > 0.0 {
        transform.translation.y = max_y;
        velocity.y = -velocity.y;
    } else if transform.translation.y < -max_y && velocity.y < 0.0 {
        transform.translation.y = -max_y;
        velocity.y = -velocity.y;
    }
}

/// Checks AABB collision between the ball and both paddles
fn collide_with_paddles(
    ball_query: Single<(&Transform, &mut Velocity, &Collider), With<Ball>>,
    paddle_query: Query<(&Transform, &Collider), (With<Paddle>, Without<Ball>)>,
) {
    let (ball_transform, mut ball_vel, ball_collider) = ball_query.into_inner();
    let ball_aabb = ball_collider.aabb(ball_transform.translation);

    for (paddle_transform, paddle_collider) in &paddle_query {
        let paddle_aabb = paddle_collider.aabb(paddle_transform.translation);

        let Some(side) = check_aabb_collision(ball_aabb, paddle_aabb) else {
            continue;
        };

        match side {
            CollisionSide::Left if ball_vel.x > 0.0 => {
                // Ball hit the front face of the Right paddle
                apply_paddle_bounce(
                    &mut ball_vel,
                    ball_transform.translation.y,
                    paddle_transform.translation.y,
                    -1.0,
                );
            }
            CollisionSide::Right if ball_vel.x < 0.0 => {
                // Ball hit the front face of the Left paddle
                apply_paddle_bounce(
                    &mut ball_vel,
                    ball_transform.translation.y,
                    paddle_transform.translation.y,
                    1.0,
                );
            }
            CollisionSide::Top if ball_vel.y < 0.0 => ball_vel.y = -ball_vel.y,
            CollisionSide::Bottom if ball_vel.y > 0.0 => ball_vel.y = -ball_vel.y,
            _ => {}
        }
    }
}

/// Redirects the ball based on where it struck the paddle and increases speed slightly
fn apply_paddle_bounce(ball_vel: &mut Vec2, ball_y: f32, paddle_y: f32, new_x_sign: f32) {
    let speed = (ball_vel.length() * BALL_SPEED_INCREMENT).min(1200.0);
    // Normalized offset from paddle center in [-1.0, 1.0]
    let hit_factor = ((ball_y - paddle_y) / (PADDLE_HEIGHT * 0.5)).clamp(-1.0, 1.0);
    let bounce_dir = Vec2::new(new_x_sign, hit_factor * 0.75).normalize();
    *ball_vel = bounce_dir * speed;
}

/// Triggers PointScored when the ball crosses the left or right boundary
fn check_goal_scored(mut commands: Commands, ball_transform: Single<&Transform, With<Ball>>) {
    let goal_x = ARENA_WIDTH * 0.5;
    let x = ball_transform.translation.x;

    if x > goal_x {
        commands.trigger(PointScored {
            scorer: PlayerSide::Left,
        });
    } else if x < -goal_x {
        commands.trigger(PointScored {
            scorer: PlayerSide::Right,
        });
    }
}

/// Observer: Resets the ball to center and serves toward the player who was just scored on
fn on_point_scored_reset_ball(
    point: On<PointScored>,
    ball_query: Single<(&mut Transform, &mut Velocity), With<Ball>>,
) {
    let (mut transform, mut velocity) = ball_query.into_inner();
    transform.translation = Vec3::new(0.0, 0.0, 1.0);

    let serve_x = match point.scorer {
        PlayerSide::Left => 1.0,
        PlayerSide::Right => -1.0,
    };
    **velocity = Vec2::new(serve_x, 0.35).normalize() * INITIAL_BALL_SPEED;
}
