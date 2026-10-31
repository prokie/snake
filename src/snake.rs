use crate::board::GridPosition;
use crate::constants::*;
use bevy::prelude::*;
use std::collections::VecDeque;

const MOVE_INTERVAL: f32 = 0.11;
const MAX_QUEUED_INPUTS: usize = 2;

pub struct SnakePlugin;

impl Plugin for SnakePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(MovementTimer(Timer::from_seconds(
            MOVE_INTERVAL,
            TimerMode::Repeating,
        )))
        .add_systems(Startup, spawn_snake)
        .add_systems(
            Update,
            (read_direction, tick_snake_movement, orient_snake_head).chain(),
        );
    }
}

#[derive(Resource, Deref, DerefMut)]
struct MovementTimer(Timer);

#[derive(Component)]
pub struct SnakeHead {
    pub direction: Direction,
    pub input_queue: VecDeque<Direction>,
}

impl Default for SnakeHead {
    fn default() -> Self {
        Self {
            direction: Direction::Right,
            input_queue: VecDeque::with_capacity(MAX_QUEUED_INPUTS),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl Direction {
    pub fn vector(self) -> IVec2 {
        match self {
            Self::Left => IVec2::NEG_X,
            Self::Right => IVec2::X,
            Self::Up => IVec2::Y,
            Self::Down => IVec2::NEG_Y,
        }
    }

    pub fn is_opposite(self, other: Self) -> bool {
        self.vector() == -other.vector()
    }
}

fn spawn_snake(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Sprite {
            image: asset_server.load("head.png"),
            custom_size: Some(Vec2::splat(CELL_SIZE)),
            ..default()
        },
        Transform::default(),
        GridPosition::center(),
        SnakeHead::default(),
    ));
}

fn read_direction(keyboard_input: Res<ButtonInput<KeyCode>>, mut head: Single<&mut SnakeHead>) {
    let candidates = [
        (KeyCode::ArrowLeft, Direction::Left),
        (KeyCode::ArrowRight, Direction::Right),
        (KeyCode::ArrowUp, Direction::Up),
        (KeyCode::ArrowDown, Direction::Down),
    ];

    for (key, dir) in candidates {
        if keyboard_input.just_pressed(key) && head.input_queue.len() < MAX_QUEUED_INPUTS {
            // Check against the last queued turn (or current direction if queue is empty)
            let last_planned_dir = head.input_queue.back().copied().unwrap_or(head.direction);

            if dir != last_planned_dir && !last_planned_dir.is_opposite(dir) {
                head.input_queue.push_back(dir);
            }
        }
    }
}

fn tick_snake_movement(
    time: Res<Time>,
    mut timer: ResMut<MovementTimer>,
    query: Single<(&mut SnakeHead, &mut GridPosition)>,
) {
    if !timer.tick(time.delta()).just_finished() {
        return;
    }

    let (mut head, mut pos) = query.into_inner();

    // Pop the next buffered turn if the player pressed any keys
    if let Some(next_dir) = head.input_queue.pop_front() {
        head.direction = next_dir;
    }

    let next_pos = GridPosition(**pos + head.direction.vector());
    *pos = next_pos.wrap();
}

fn orient_snake_head(
    head_query: Single<(&SnakeHead, &mut Transform, &mut Sprite), Changed<GridPosition>>,
) {
    let (head, mut transform, mut sprite) = head_query.into_inner();
    let angle = match head.direction {
        Direction::Right | Direction::Left => 0.0,
        Direction::Up => std::f32::consts::FRAC_PI_2,
        Direction::Down => -std::f32::consts::FRAC_PI_2,
    };
    transform.rotation = Quat::from_rotation_z(angle);
    sprite.flip_x = matches!(head.direction, Direction::Left);
}
