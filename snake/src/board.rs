use crate::constants::*;
use bevy::prelude::*;

pub struct BoardPlugin;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_board_background)
            // Runs in PostUpdate so any GridPosition changes from Update are synced before rendering
            .add_systems(PostUpdate, sync_grid_positions);
    }
}

#[derive(Component, Clone, Copy, Debug, PartialEq, Eq, Deref, DerefMut)]
#[component(summary_tick)]
pub struct GridPosition(pub IVec2);

impl GridPosition {
    pub fn new(x: i32, y: i32) -> Self {
        Self(IVec2::new(x, y))
    }

    /// Center tile of the board
    pub fn center() -> Self {
        Self::new(BOARD_WIDTH / 2, BOARD_HEIGHT / 2)
    }

    /// Checks if the coordinate is inside the playable grid (0..BOARD_WIDTH, 0..BOARD_HEIGHT)
    pub fn in_bounds(self) -> bool {
        self.x >= 0 && self.x < BOARD_WIDTH && self.y >= 0 && self.y < BOARD_HEIGHT
    }

    /// Wraps coordinates around the edges (Pac-Man style) if you want wrap-around rules
    pub fn wrap(self) -> Self {
        Self(IVec2::new(
            self.x.rem_euclid(BOARD_WIDTH),
            self.y.rem_euclid(BOARD_HEIGHT),
        ))
    }

    /// Converts grid (x, y) into centered window pixel coordinates
    pub fn to_world_xyz(self, z: f32) -> Vec3 {
        let board_pixel_w = BOARD_WIDTH as f32 * CELL_SIZE;
        let board_pixel_h = BOARD_HEIGHT as f32 * CELL_SIZE;

        // Shift from bottom-left (0,0) to centered camera space, plus half a cell to center the sprite
        let x = (self.x as f32 * CELL_SIZE) - (board_pixel_w / 2.0) + (CELL_SIZE / 2.0);
        let y = (self.y as f32 * CELL_SIZE) - (board_pixel_h / 2.0) + (CELL_SIZE / 2.0);

        Vec3::new(x, y, z)
    }
}

/// Spawns a dark background rectangle behind the board to make it easier to see the grid and sprites.
fn spawn_board_background(mut commands: Commands) {
    let width = BOARD_WIDTH as f32 * CELL_SIZE;
    let height = BOARD_HEIGHT as f32 * CELL_SIZE;

    commands.spawn((
        Sprite {
            color: Color::srgb(0.12, 0.12, 0.15),
            custom_size: Some(Vec2::new(width, height)),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, -1.0),
    ));
}

/// Automatically converts GridPosition to pixel Transform for ALL grid entities
fn sync_grid_positions(mut query: Query<(&GridPosition, &mut Transform), Changed<GridPosition>>) {
    for (pos, mut transform) in &mut query {
        // Preserve whatever Z-layer the entity was spawned with
        transform.translation = pos.to_world_xyz(transform.translation.z);
    }
}
