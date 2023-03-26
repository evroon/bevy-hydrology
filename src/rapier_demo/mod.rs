mod systems;
use systems::*;

use bevy::prelude::*;

pub struct RapierDemoPlugin;

impl Plugin for RapierDemoPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_rigid_bodies)
            .add_system(spawn_on_mouseclick);
    }
}
