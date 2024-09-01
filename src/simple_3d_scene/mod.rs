mod system;
use bevy_egui::EguiPlugin;
use system::{simple_3d_scene, ui_system};

use bevy::prelude::*;

pub struct Simple3DScenePlugin;

impl Plugin for Simple3DScenePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, simple_3d_scene)
            .add_systems(Update, ui_system)
            .add_plugins(EguiPlugin);
    }
}
