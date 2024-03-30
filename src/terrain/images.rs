use super::TERRAIN_SIZE;
use bevy::{
    prelude::*,
    render::{
        render_asset::RenderAssetUsages,
        render_resource::{Extent3d, TextureDimension, TextureFormat, TextureUsages},
    },
};

pub fn build_images(
    mut images: ResMut<Assets<Image>>,
) -> (Handle<Image>, Handle<Image>, Handle<Image>) {
    let mut heightmap_image = Image::new_fill(
        Extent3d {
            width: TERRAIN_SIZE.x,
            height: TERRAIN_SIZE.y,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        &[0; 4 * 2],
        TextureFormat::R32Float,
        RenderAssetUsages::RENDER_WORLD,
    );
    heightmap_image.texture_descriptor.usage =
        TextureUsages::COPY_DST | TextureUsages::STORAGE_BINDING | TextureUsages::TEXTURE_BINDING;

    let mut normalmap_topright_image = Image::new_fill(
        Extent3d {
            width: TERRAIN_SIZE.x,
            height: TERRAIN_SIZE.y,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        &[0; 4 * 4 * 2],
        TextureFormat::Rgba32Float,
        RenderAssetUsages::RENDER_WORLD,
    );
    normalmap_topright_image.texture_descriptor.usage =
        TextureUsages::COPY_DST | TextureUsages::STORAGE_BINDING | TextureUsages::TEXTURE_BINDING;

    let mut normalmap_bottomleft_image = Image::new_fill(
        Extent3d {
            width: TERRAIN_SIZE.x,
            height: TERRAIN_SIZE.y,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        &[0; 4 * 4 * 2],
        TextureFormat::Rgba32Float,
        RenderAssetUsages::RENDER_WORLD,
    );
    normalmap_bottomleft_image.texture_descriptor.usage =
        TextureUsages::COPY_DST | TextureUsages::STORAGE_BINDING | TextureUsages::TEXTURE_BINDING;

    (
        images.add(heightmap_image),
        images.add(normalmap_topright_image),
        images.add(normalmap_bottomleft_image),
    )
}
