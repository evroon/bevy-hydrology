mod system;
use system::*;

use bevy::prelude::*;

pub const CUBE_SIZE: bevy::prelude::Vec3 = Vec3::new(10.0, 5.0, 10.0);

pub struct CubesPlugin;

impl Plugin for CubesPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_cubes_scene);
    }
}
