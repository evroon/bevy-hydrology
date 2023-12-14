mod system;
use system::*;

use bevy::prelude::*;

pub const TERRAIN_SIZE: bevy::prelude::UVec2 = UVec2::new(256, 256);
pub const CELL_SIZE: f32 = 1.0;

pub struct LowPolyTerrainPlugin;

impl Plugin for LowPolyTerrainPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_low_poly_terrain);
    }
}
