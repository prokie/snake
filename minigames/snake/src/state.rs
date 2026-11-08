use bevy::prelude::*;

pub struct StatePlugin;

impl Plugin for StatePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>()
            .add_systems(OnEnter(GameState::GameOver), spawn_game_over_ui)
            .add_systems(
                Update,
                restart_on_r.run_if(in_state(GameState::GameOver)),
            );
    }
}

#[derive(States, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum GameState {
    #[default]
    Playing,
    GameOver,
}

fn spawn_game_over_ui(mut commands: Commands) {
    commands.spawn((
        DespawnOnExit(GameState::GameOver),
        Text2d::new("GAME OVER\nPress R to Restart"),
        TextFont {
            font_size: FontSize::Px(24.0),
            ..default()
        },
        TextColor(Color::srgb(1.0, 0.3, 0.3)),
        TextLayout::justify(Justify::Center),
        Transform::from_xyz(0.0, 0.0, 10.0),
    ));
}

fn restart_on_r(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if keyboard_input.just_pressed(KeyCode::KeyR) {
        next_state.set(GameState::Playing);
    }
}