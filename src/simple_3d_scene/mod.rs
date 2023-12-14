mod system;
use system::*;

use bevy::prelude::*;

pub const BOX_SIZE: bevy::prelude::Vec3 = Vec3::new(10.0, 5.0, 10.0);

pub struct Simple3DScenePlugin;

impl Plugin for Simple3DScenePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (simple_3d_scene, ui_system));
    }
}
