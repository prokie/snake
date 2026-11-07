use crate::board::GridPosition;
use crate::constants::*;
use crate::food::FoodEaten;
use crate::state::GameState;
use bevy::prelude::*;
use std::collections::VecDeque;

const MOVE_INTERVAL: f32 = 0.11;
const MAX_QUEUED_INPUTS: usize = 2;
const INITIAL_TAIL_LENGTH: i32 = 2;

const SEGMENT_SIZE: f32 = CELL_SIZE - 2.0;
const SEGMENT_COLOR: Color = Color::srgb(0.25, 0.80, 0.35);

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct SnakeSystems;

pub struct SnakePlugin;

impl Plugin for SnakePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(MovementTimer(Timer::from_seconds(
            MOVE_INTERVAL,
            TimerMode::Repeating,
        )))
        .init_resource::<SnakeSegments>()
        .add_systems(OnEnter(GameState::Playing), spawn_snake)
        .add_systems(
            Update,
            (
                read_direction,
                tick_snake_movement,
                check_self_collision,
                orient_snake_head,
            )
                .chain()
                .run_if(in_state(GameState::Playing)),
        )
        .add_observer(on_food_eaten_grow_snake);
    }
}

#[derive(Resource, Deref, DerefMut)]
struct MovementTimer(Timer);

/// Ordered list of body segment entities (index 0 is directly behind the head)
#[derive(Resource, Default, Deref, DerefMut)]
pub struct SnakeSegments(pub Vec<Entity>);

/// Marker component for trailing body squares
#[derive(Component)]
pub struct SnakeSegment;

#[derive(Component)]
pub struct SnakeHead {
    pub direction: Direction,
    pub input_queue: VecDeque<Direction>,
    pub pending_growth: usize,
}

impl Default for SnakeHead {
    fn default() -> Self {
        Self {
            direction: Direction::Right,
            input_queue: VecDeque::with_capacity(MAX_QUEUED_INPUTS),
            pending_growth: 0,
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

fn spawn_snake(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut segments: ResMut<SnakeSegments>,
    mut timer: ResMut<MovementTimer>,
) {
    segments.clear();
    timer.reset();

    let start_pos = GridPosition::center();

    // Spawn Head (automatically despawned next time GameState::Playing is entered)
    commands.spawn((
        DespawnOnExit(GameState::Playing),
        Sprite {
            image: asset_server.load("head.png"),
            custom_size: Some(Vec2::splat(CELL_SIZE)),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 1.0),
        start_pos,
        SnakeHead::default(),
    ));

    // 2. Spawn initial green body segments
    for i in 1..=INITIAL_TAIL_LENGTH {
        let seg_pos = GridPosition::new(start_pos.x - i, start_pos.y);
        let entity = spawn_segment(&mut commands, seg_pos);
        segments.push(entity);
    }
}

/// Helper that spawns a single green square body segment at a GridPosition
fn spawn_segment(commands: &mut Commands, pos: GridPosition) -> Entity {
    commands
        .spawn((
            DespawnOnExit(GameState::Playing),
            Sprite {
                color: SEGMENT_COLOR,
                custom_size: Some(Vec2::splat(SEGMENT_SIZE)),
                ..default()
            },
            Transform::from_xyz(0.0, 0.0, 0.5),
            pos,
            SnakeSegment,
        ))
        .id()
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
            let last_planned_dir = head.input_queue.back().copied().unwrap_or(head.direction);

            if dir != last_planned_dir && !last_planned_dir.is_opposite(dir) {
                head.input_queue.push_back(dir);
            }
        }
    }
}

fn tick_snake_movement(
    mut commands: Commands,
    time: Res<Time>,
    mut timer: ResMut<MovementTimer>,
    mut segments: ResMut<SnakeSegments>,
    head_query: Single<(&mut SnakeHead, &mut GridPosition), Without<SnakeSegment>>,
    mut segment_positions: Query<&mut GridPosition, With<SnakeSegment>>,
) {
    if !timer.tick(time.delta()).just_finished() {
        return;
    }

    let (mut head, mut head_pos) = head_query.into_inner();

    // Save the head's current position before stepping forward
    let prev_head_pos = *head_pos;

    // Snapshot current positions of all body segments in order
    let prev_segment_positions: Vec<GridPosition> = segments
        .iter()
        .filter_map(|&entity| segment_positions.get(entity).ok().copied())
        .collect();

    // Move the head one grid step in the planned direction
    if let Some(next_dir) = head.input_queue.pop_front() {
        head.direction = next_dir;
    }
    let next_pos = GridPosition(**head_pos + head.direction.vector());
    *head_pos = next_pos.wrap();

    // Move each segment to the position previously occupied by the piece ahead of it
    let mut target_pos = prev_head_pos;
    for (&entity, &old_pos) in segments.iter().zip(prev_segment_positions.iter()) {
        if let Ok(mut seg_pos) = segment_positions.get_mut(entity) {
            *seg_pos = target_pos;
        }
        target_pos = old_pos;
    }

    // If the snake ate food, spawn a new green square at the vacated tail tip
    if head.pending_growth > 0 {
        let new_segment = spawn_segment(&mut commands, target_pos);
        segments.push(new_segment);
        head.pending_growth -= 1;
    }
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

fn check_self_collision(
    head_query: Single<&GridPosition, (With<SnakeHead>, Changed<GridPosition>)>,
    body_query: Query<&GridPosition, With<SnakeSegment>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    let head_pos = head_query.into_inner();

    if body_query.iter().any(|body_pos| body_pos == head_pos) {
        next_state.set(GameState::GameOver);
    }
}

fn on_food_eaten_grow_snake(food_eaten: On<FoodEaten>, mut head: Single<&mut SnakeHead>) {
    head.pending_growth += food_eaten.growth;
}
