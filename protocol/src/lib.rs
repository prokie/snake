use bevy::curve::Curve;
use bevy::ecs::entity::MapEntities;
use bevy::prelude::*;
use bevy::state::app::AppExtStates;
use lightyear::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(States, Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
pub enum GameMode {
    #[default]
    Lobby,
    Board,
    Pong,
    Snake,
}

#[derive(Component, Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq)]
pub struct PlayerId(pub PeerId);

#[derive(Component, Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub struct PlayerColor(pub Color);

/// Shared 2D position component that supports Lightyear prediction + linear interpolation.
#[derive(
    Component,
    Serialize,
    Deserialize,
    Clone,
    Copy,
    Debug,
    Default,
    PartialEq,
    Reflect,
    Deref,
    DerefMut,
)]
pub struct Position2D(pub Vec2);

impl Ease for Position2D {
    fn interpolating_curve_unbounded(start: Self, end: Self) -> impl Curve<Self> {
        FunctionCurve::new(Interval::UNIT, move |t| {
            Position2D(Vec2::lerp(start.0, end.0, t))
        })
    }
}

pub struct GameFlowChannel;

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq)]
pub struct ChangeGameMode(pub GameMode);

#[derive(Serialize, Deserialize, Debug, Default, PartialEq, Eq, Clone, Copy, Reflect)]
pub struct Direction {
    pub up: bool,
    pub down: bool,
    pub left: bool,
    pub right: bool,
    pub action: bool,
}

impl Direction {
    pub fn is_none(&self) -> bool {
        !self.up && !self.down && !self.left && !self.right && !self.action
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone, Copy, Reflect)]
pub enum Inputs {
    Direction(Direction),
}

impl Default for Inputs {
    fn default() -> Self {
        Self::Direction(Direction::default())
    }
}

impl MapEntities for Inputs {
    fn map_entities<M: EntityMapper>(&mut self, _entity_mapper: &mut M) {}
}

#[derive(Clone)]
pub struct ProtocolPlugin;

impl Plugin for ProtocolPlugin {
    fn build(&self, app: &mut App) {
        // Global state machine
        app.init_state::<GameMode>();

        // Messages
        app.register_message::<ChangeGameMode>()
            .add_direction(NetworkDirection::ServerToClient);

        // Inputs
        app.add_plugins(input::native::InputPlugin::<Inputs>::default());

        // Replicated Components
        app.component::<PlayerId>().replicate();
        app.component::<PlayerColor>().replicate();

        app.component::<Position2D>()
            .replicate()
            .predict()
            .add_linear_interpolation();

        // Channels
        app.add_channel::<GameFlowChannel>(ChannelSettings {
            mode: ChannelMode::OrderedReliable(ReliableSettings::default()),
            ..default()
        })
        .add_direction(NetworkDirection::ServerToClient);
    }
}
