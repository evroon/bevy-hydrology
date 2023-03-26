mod components;
mod systems;
use systems::*;

use bevy::prelude::*;

pub const BOID_SIZE: bevy::prelude::Vec2 = Vec2::new(0.15, 0.075); // height, radius
pub const BOID_COUNT: i32 = 40;
pub const BOID_MARGIN_FROM_EDGE: f32 = BOID_SIZE.x / 2.0 + BOID_SIZE.y;

pub struct BoidsPlugin;

impl Plugin for BoidsPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_boids).add_system(update_boids);
    }
}
