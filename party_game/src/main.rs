use bevy::{prelude::*, window::WindowResolution};
use lightyear::prelude::{client::ClientPlugins, server::ServerPlugins};
use pong::PongPlugin;
use protocol::{GameMode, ProtocolPlugin};
use std::time::Duration;

const FIXED_TIMESTEP_HZ: f64 = 64.0;

fn main() {
    let tick_duration = Duration::from_secs_f64(1.0 / FIXED_TIMESTEP_HZ);

    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Pontus Party".into(),
                        resolution: WindowResolution::new(1920, 1080),
                        resizable: true,
                        ..default()
                    }),
                    ..default()
                }),
        )
        // Initialize Lightyear core plugins BEFORE ProtocolPlugin
        .add_plugins((
            ClientPlugins { tick_duration },
            ServerPlugins { tick_duration },
        ))
        // Shared protocol + Pong minigame plugin
        .add_plugins((ProtocolPlugin, PongPlugin))
        // Temporary Lobby UI & state switcher for testing
        .add_systems(OnEnter(GameMode::Lobby), setup_lobby)
        .add_systems(Update, switch_game_mode)
        .run();
}

fn setup_lobby(mut commands: Commands) {
    commands.insert_resource(ClearColor(Color::srgb(0.08, 0.09, 0.12)));

    commands.spawn((DespawnOnExit(GameMode::Lobby), Camera2d));

    commands.spawn((
        DespawnOnExit(GameMode::Lobby),
        Text2d::new(
            "PONTUS PARTY - LOBBY\n\nPress [1] to start Pong\nPress [Esc] to return to Lobby",
        ),
        TextFont {
            font_size: FontSize::Px(32.0),
            ..default()
        },
        TextColor(Color::WHITE),
        TextLayout::justify(Justify::Center),
    ));
}

fn switch_game_mode(
    keyboard: Res<ButtonInput<KeyCode>>,
    current_mode: Res<State<GameMode>>,
    mut next_mode: ResMut<NextState<GameMode>>,
) {
    if keyboard.just_pressed(KeyCode::Digit1) && *current_mode.get() != GameMode::Pong {
        info!("Switching to GameMode::Pong");
        next_mode.set(GameMode::Pong);
    } else if keyboard.just_pressed(KeyCode::Escape) && *current_mode.get() != GameMode::Lobby {
        info!("Returning to GameMode::Lobby");
        next_mode.set(GameMode::Lobby);
    }
}
