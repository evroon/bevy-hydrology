use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::camera_control::CameraController;

use super::BOX_SIZE;

pub fn simple_3d_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let camera_controller = CameraController::default();

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

    // light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(2.0, 4.5, 1.0),
        ..default()
    });
    // camera
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(-BOX_SIZE.x * 0.9, BOX_SIZE.y * 1.5, BOX_SIZE.z)
                .looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        camera_controller,
    ));
}
