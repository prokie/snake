use bevy::prelude::*;

use crate::constants::*;
use crate::physics::{Collider, PhysicsSystems, Velocity};
use crate::state::GameState;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct PaddleSystems;

pub struct PaddlePlugin;

impl Plugin for PaddlePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), spawn_paddles)
            .add_systems(
                Update,
                read_paddle_input
                    .before(PhysicsSystems)
                    .run_if(in_state(GameState::Playing)),
            )
            .add_systems(
                Update,
                clamp_paddles
                    .in_set(PaddleSystems)
                    .after(PhysicsSystems)
                    .run_if(in_state(GameState::Playing)),
            );
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PlayerSide {
    Left,
    Right,
}

#[derive(Component)]
#[require(Velocity, Collider)]
pub struct Paddle {
    pub side: PlayerSide,
    pub up_key: KeyCode,
    pub down_key: KeyCode,
}

fn spawn_paddles(mut commands: Commands) {
    let configs = [
        (
            PlayerSide::Left,
            -PADDLE_X_OFFSET,
            KeyCode::KeyW,
            KeyCode::KeyS,
        ),
        (
            PlayerSide::Right,
            PADDLE_X_OFFSET,
            KeyCode::ArrowUp,
            KeyCode::ArrowDown,
        ),
    ];

    for (side, x, up_key, down_key) in configs {
        commands.spawn((
            DespawnOnExit(GameState::Playing),
            Sprite {
                color: Color::WHITE,
                custom_size: Some(Vec2::new(PADDLE_WIDTH, PADDLE_HEIGHT)),
                ..default()
            },
            Transform::from_xyz(x, 0.0, 1.0),
            Collider::new(PADDLE_WIDTH, PADDLE_HEIGHT),
            Paddle {
                side,
                up_key,
                down_key,
            },
        ));
    }
}

fn read_paddle_input(input: Res<ButtonInput<KeyCode>>, mut query: Query<(&Paddle, &mut Velocity)>) {
    for (paddle, mut velocity) in &mut query {
        let mut dir = 0.0;
        if input.pressed(paddle.up_key) {
            dir += 1.0;
        }
        if input.pressed(paddle.down_key) {
            dir -= 1.0;
        }
        velocity.y = dir * PADDLE_SPEED;
    }
}

fn clamp_paddles(mut query: Query<&mut Transform, With<Paddle>>) {
    let max_y = (ARENA_HEIGHT * 0.5) - (PADDLE_HEIGHT * 0.5);
    for mut transform in &mut query {
        transform.translation.y = transform.translation.y.clamp(-max_y, max_y);
    }
}
