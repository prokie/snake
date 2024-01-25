use bevy::{prelude::*, sprite::MaterialMesh2dBundle, window::PrimaryWindow};

enum Direction {
    Left,
    Right,
    Up,
    Down,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (spawn_snake, spawn_camera))
        .add_systems(Update, snake_movement)
        .run();
}

#[derive(Component)]
pub struct Snake {
    pub direction: Direction,
}

fn spawn_snake(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();

    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes.add(shape::Circle::new(50.).into()).into(),
        material: materials.add(ColorMaterial::from(Color::PURPLE)),
        transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
        ..default()
    });
}

fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.get_single().unwrap();
    commands.spawn((
        Camera2dBundle {
            transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
            ..default()
        },
        Snake {
            direction: Direction::Up,
        },
    ));
}

pub const PLAYER_SPEED: f32 = 200.0;

fn snake_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut snake_transform_query: Query<&mut Transform, With<Snake>>,
    mut snake_query: Query<&mut Snake>,
    time: Res<Time>,
) {
    if let Ok(mut transform) = snake_transform_query.get_single_mut() {
        let mut snake = snake_query
            .get_single_mut()
            .expect("Error: Could not find a single player.");

        if keyboard_input.pressed(KeyCode::Left) {
            snake.direction = Direction::Left;
        }
        if keyboard_input.pressed(KeyCode::Right) {
            snake.direction = Direction::Right;
        }
        if keyboard_input.pressed(KeyCode::Up) {
            snake.direction = Direction::Up;
        }
        if keyboard_input.pressed(KeyCode::Down) {
            snake.direction = Direction::Down;
        }

        let direction = match snake.direction {
            Direction::Left => Vec3::new(1.0, 0.0, 0.0),
            Direction::Right => Vec3::new(-1.0, 0.0, 0.0),
            Direction::Up => Vec3::new(0.0, -1.0, 0.0),
            Direction::Down => Vec3::new(0.0, 1.0, 0.0),
        };

        transform.translation += direction * PLAYER_SPEED * time.delta_seconds();
    }
}
