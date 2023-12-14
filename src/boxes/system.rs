use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use super::CUBE_SIZE;

pub fn setup_cubes_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mut plane_transform = Transform::from_xyz(0.0, -1.0, 0.0);
    plane_transform.rotate_x(-0.1);

    commands
        .spawn(PbrBundle {
            mesh: meshes.add(shape::Plane::from_size(10.0).into()),
            material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
            ..default()
        })
        .insert(Collider::cuboid(100.0, 0.1, 100.0))
        .insert(TransformBundle::from(plane_transform));

    // cubes
    commands
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Box::new(
                BOX_SIZE.x, BOX_SIZE.y, BOX_SIZE.z,
            ))),
            material: materials.add(Color::rgba(0.8, 0.7, 0.6, 1.0).into()),
            ..default()
        })
        .insert(Collider::cuboid(
            BOX_SIZE.x / 2.0,
            BOX_SIZE.y / 2.0,
            BOX_SIZE.z / 2.0,
        ))
        .insert(TransformBundle::from(Transform::from_xyz(
            BOX_SIZE.x, 0.0, 0.0,
        )));

    commands
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Box::new(
                BOX_SIZE.x, BOX_SIZE.y, BOX_SIZE.z,
            ))),
            material: materials.add(Color::rgba(0.8, 0.7, 0.6, 1.0).into()),
            ..default()
        })
        .insert(Collider::cuboid(
            BOX_SIZE.x / 2.0,
            BOX_SIZE.y / 2.0,
            BOX_SIZE.z / 2.0,
        ))
        .insert(TransformBundle::from(Transform::from_xyz(
            -BOX_SIZE.x,
            0.0,
            0.0,
        )));

    commands
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Box::new(
                BOX_SIZE.x, BOX_SIZE.y, BOX_SIZE.z,
            ))),
            material: materials.add(Color::rgba(0.8, 0.7, 0.6, 1.0).into()),
            ..default()
        })
        .insert(Collider::cuboid(
            BOX_SIZE.x / 2.0,
            BOX_SIZE.y / 2.0,
            BOX_SIZE.z / 2.0,
        ))
        .insert(TransformBundle::from(Transform::from_xyz(
            0.0,
            0.0,
            -BOX_SIZE.z,
        )));
}
