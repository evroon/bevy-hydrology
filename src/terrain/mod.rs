mod system;
use system::*;
mod hydrology;
mod ui;

use bevy::{pbr::ExtendedMaterial, prelude::*};

use self::{hydrology::hydrology_system, ui::ui_system};

pub const TERRAIN_SIZE: bevy::prelude::UVec2 = UVec2::new(256, 256);
pub const TERRAIN_SIZE_F32: bevy::prelude::Vec2 =
    Vec2::new(TERRAIN_SIZE.x as f32, TERRAIN_SIZE.y as f32);
pub const _TERRAIN_SIZE_I32: bevy::prelude::IVec2 =
    IVec2::new(TERRAIN_SIZE.x as i32, TERRAIN_SIZE.y as i32);

pub const CELL_SIZE: f32 = 1.0;

pub struct LowPolyTerrainPlugin;

impl Plugin for LowPolyTerrainPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(MaterialPlugin::<
            ExtendedMaterial<StandardMaterial, TerrainShaderExtension>,
        >::default())
            .add_systems(Startup, setup_low_poly_terrain)
            .add_systems(Update, ui_system)
            .add_systems(Update, hydrology_system);
    }
}
