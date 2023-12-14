use super::{CELL_SIZE, TERRAIN_SIZE};
use bevy::{
    prelude::*,
    render::{mesh, render_resource::PrimitiveTopology},
};
use noise::{NoiseFn, Perlin};

fn create_mesh(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let vertex_count = (TERRAIN_SIZE.x * TERRAIN_SIZE.y) as usize;

    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
    let mut positions = vec![[0., 0., 0.]; vertex_count];
    let mut normals = vec![[0., 0., 0.]; vertex_count];
    let mut indices = vec![0u32; (((TERRAIN_SIZE.x - 1) * (TERRAIN_SIZE.y - 1)) as usize) * 6];

    let perlin = Perlin::new(123);
    let vert_scale = 5.0;
    let horiz_scale: f32 = 0.0523;

    for i_32 in 0..TERRAIN_SIZE.x * TERRAIN_SIZE.y {
        let x_pos = (i_32 % TERRAIN_SIZE.x) as f32 - TERRAIN_SIZE.x as f32 / 2.0;
        let z_pos = (i_32 / TERRAIN_SIZE.x) as f32 - TERRAIN_SIZE.y as f32 / 2.0;

        let i = usize::try_from(i_32).unwrap();
        let perlin_pos = [(x_pos * horiz_scale) as f64, (z_pos * horiz_scale) as f64];

        positions[i] = [x_pos, vert_scale * perlin.get(perlin_pos) as f32, z_pos];
    }
    for i_32 in 0..TERRAIN_SIZE.x * TERRAIN_SIZE.y {
        let i = usize::try_from(i_32).unwrap();

        let v1 = Vec3::from_array(positions[i]);
        let v2 = Vec3::from_array(
            positions
                .get(i + 1)
                .unwrap_or(&[v1[0] + 1., v1[1], v1[2]])
                .clone(),
        );
        let v3 = Vec3::from_array(
            positions
                .get(i + (TERRAIN_SIZE.x as usize))
                .unwrap_or(&[v1[0], v1[1], v1[2] + 1.0])
                .clone(),
        );

        normals[i] = (v3 - v1).cross(v2 - v1).to_array();
    }

    for i_32 in 0..(TERRAIN_SIZE.x  * TERRAIN_SIZE.y) {
        let x_pos = i_32 % TERRAIN_SIZE.x;
        let z_pos = i_32 / TERRAIN_SIZE.x;

        if x_pos >= TERRAIN_SIZE.x - 1 || z_pos >= TERRAIN_SIZE.y - 1 {
            continue;
        }
        let i_idx_usize = usize::try_from((x_pos + z_pos * (TERRAIN_SIZE.x as u32 - 1)) * 6).unwrap();

        let slice = &[
            i_32 + 0,
            i_32 + TERRAIN_SIZE.x,
            i_32 + TERRAIN_SIZE.x + 1,
            i_32 + 0,
            i_32 + TERRAIN_SIZE.x + 1,
            i_32 + 1,
        ];
        indices.splice(i_idx_usize..i_idx_usize + 6, slice.iter().cloned());
    }

    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, vec![[0., 0.]; vertex_count]);
    mesh.set_indices(Some(mesh::Indices::U32(indices)));

    commands.spawn(PbrBundle {
        mesh: meshes.add(mesh),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..default()
    });
}

pub fn setup_low_poly_terrain(
    commands: Commands,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<StandardMaterial>>,
) {
    create_mesh(commands, meshes, materials);
}
