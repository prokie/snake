use bevy::{prelude::*, sprite::MaterialMesh2dBundle, window::PrimaryWindow};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (spawn_snake, spawn_camera))
        .add_systems(Update, snake_movement)
        .run();
}

#[derive(Component)]
pub struct Snake {}

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
        Snake {},
    ));
}

pub const PLAYER_SPEED: f32 = 100.0;

fn snake_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut snake_query: Query<&mut Transform, With<Snake>>,
    time: Res<Time>,
) {
    if let Ok(mut transform) = snake_query.get_single_mut() {
        let mut direction = Vec3::ZERO;

        if keyboard_input.pressed(KeyCode::Left) {
            direction += Vec3::new(50.0, 0.0, 0.0)
        }
        if keyboard_input.pressed(KeyCode::Right) {
            direction += Vec3::new(-50.0, 0.0, 0.0)
        }
        if keyboard_input.pressed(KeyCode::Up) {
            direction += Vec3::new(0.0, -50.0, 0.0)
        }
        if keyboard_input.pressed(KeyCode::Down) {
            direction += Vec3::new(0.0, 50.0, 0.0)
        }

        if direction.length() > 0.0 {
            direction = direction.normalize();
        }

        transform.translation += direction * PLAYER_SPEED * time.delta_seconds();
    }
}
