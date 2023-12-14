mod systems;
use systems::*;

use bevy::prelude::*;

pub struct RapierDemoPlugin;

impl Plugin for RapierDemoPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_rigid_bodies)
            .add_systems(Update, spawn_on_mouseclick);
    }
}
