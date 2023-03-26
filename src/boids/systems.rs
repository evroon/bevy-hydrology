use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use super::{components::BoidComponent, BOID_MARGIN_FROM_EDGE};
use rand::{rngs::ThreadRng, thread_rng, Rng};

use crate::simple_3d_scene::BOX_SIZE;

use super::{BOID_COUNT, BOID_SIZE};

fn get_random_position_in_box(mut rng: ThreadRng) -> Transform {
    Transform::from_xyz(
        BOX_SIZE.x * (rng.gen::<f32>() - 0.5),
        BOX_SIZE.y * rng.gen::<f32>(),
        BOX_SIZE.z * (rng.gen::<f32>() - 0.5),
    )
}

pub fn spawn_boids(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let rng = thread_rng();

    for _ in 0..BOID_COUNT {
        commands
            .spawn(PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Capsule {
                    depth: BOID_SIZE.x,
                    radius: BOID_SIZE.y,
                    ..default()
                })),
                material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
                transform: get_random_position_in_box(rng.clone()),
                ..default()
            })
            .insert(RigidBody::Dynamic)
            .insert(Collider::capsule_y(BOID_SIZE.x * 0.5, BOID_SIZE.y * 0.5))
            .insert(BoidComponent);
    }
}

pub fn update_boids(mut query: Query<&mut Transform, With<BoidComponent>>, time: Res<Time>) {
    let delta_seconds = time.delta_seconds();

    for mut transform in &mut query {
        let forward = transform.clone().up();
        let velocity = forward * delta_seconds / 2.0;

        transform.translation += velocity;

        if transform.translation.x > BOX_SIZE.x - BOID_MARGIN_FROM_EDGE {
            transform.translation.x = -BOX_SIZE.x + BOID_MARGIN_FROM_EDGE;
        }
        if transform.translation.x < -BOX_SIZE.x + BOID_MARGIN_FROM_EDGE {
            transform.translation.x = BOX_SIZE.x - BOID_MARGIN_FROM_EDGE
        }
        if transform.translation.y > BOX_SIZE.y - BOID_MARGIN_FROM_EDGE {
            transform.translation.y = BOID_MARGIN_FROM_EDGE;
        }
        if transform.translation.y < BOID_MARGIN_FROM_EDGE {
            transform.translation.y = BOX_SIZE.y - BOID_MARGIN_FROM_EDGE;
        }
        if transform.translation.z > BOX_SIZE.z - BOID_MARGIN_FROM_EDGE {
            transform.translation.z = -BOX_SIZE.z + BOID_MARGIN_FROM_EDGE;
        }
        if transform.translation.z < -BOX_SIZE.z + BOID_MARGIN_FROM_EDGE {
            transform.translation.z = BOX_SIZE.z - BOID_MARGIN_FROM_EDGE
        }
    }
}
