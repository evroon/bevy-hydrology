use std::f32::consts::PI;

use bevy::prelude::*;
use bevy_egui::{
    egui::{self, Pos2, Ui},
    EguiContexts,
};
use bevy_panorbit_camera::PanOrbitCamera;

use crate::terrain::TERRAIN_SIZE_F32;

pub fn simple_3d_scene(mut commands: Commands, mut ambient_light: ResMut<AmbientLight>) {
    let mut camera_transform = Transform::from_xyz(0.0, 0.0, 0.0);
    camera_transform.rotate_x(-30.0 / 180.0 * PI);

    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: 8000.0,
            shadows_enabled: true,
            ..default()
        },
        transform: camera_transform,
        ..default()
    });
    commands.spawn((
        Camera3dBundle {
            camera: Camera {
                hdr: true,
                ..default()
            },
            transform: Transform::from_xyz(TERRAIN_SIZE_F32.x - 240.0, 240.0, TERRAIN_SIZE_F32.y)
                .looking_at(
                    Vec3::new(TERRAIN_SIZE_F32.x * 0.5, 0.0, TERRAIN_SIZE_F32.y * 0.5),
                    Vec3::Y,
                ),
            ..default()
        },
        PanOrbitCamera {
            button_pan: MouseButton::Middle,
            button_orbit: MouseButton::Left,
            ..Default::default()
        },
        FogSettings {
            color: Color::linear_rgba(0.35, 0.48, 0.66, 1.0),
            directional_light_color: Color::linear_rgba(1.0, 0.95, 0.85, 0.5),
            directional_light_exponent: 30.0,
            falloff: FogFalloff::from_visibility_colors(
                8192.0,
                Color::linear_rgb(0.35, 0.5, 0.66),
                Color::linear_rgb(0.8, 0.844, 1.0),
            ),
        },
        // ScreenSpaceAmbientOcclusionBundle::default(), Too slow for my GPU
    ));

    ambient_light.color = Color::WHITE;
    ambient_light.brightness = 200.0;
}

pub fn directional_light_ui(
    light: &mut DirectionalLight,
    camera_transform: (&Transform, &Camera),
    ui: &mut Ui,
    mut fog: Mut<FogSettings>,
) {
    ui.label("Intensity");
    ui.add(egui::Slider::new(&mut light.illuminance, 100.0..=100_000.0));
    ui.end_row();

    ui.label("Shadows");
    ui.checkbox(&mut light.shadows_enabled, "Enabled");
    ui.end_row();

    ui.label("Camera position x");
    ui.label(camera_transform.0.translation.x.round().to_string());
    ui.end_row();
    ui.label("Camera position y");
    ui.label(camera_transform.0.translation.y.round().to_string());
    ui.end_row();
    ui.label("Camera position z");
    ui.label(camera_transform.0.translation.z.round().to_string());
    ui.end_row();

    if ui.button("Toggle fog").clicked() {
        let a = fog.color.alpha();
        fog.color.set_alpha(1.0 - a);
    };
    // ui.add(egui::Slider::new(&mut fog.falloff., 100.0..=100_000.0));
    ui.end_row();
}

pub fn ui_system(
    mut light_query: Query<&mut DirectionalLight>,
    camera_query: Query<(&Transform, &Camera)>,
    mut contexts: EguiContexts,
    mut fog: Query<&mut FogSettings>,
) {
    egui::Window::new("3D world")
        .current_pos(Pos2 { x: 10., y: 10. })
        .show(contexts.ctx_mut(), |ui| {
            egui::Grid::new("3dworld_grid")
                .num_columns(2)
                .spacing([40.0, 4.0])
                .striped(true)
                .show(ui, |ui| {
                    light_query.iter_mut().for_each(|mut light| {
                        directional_light_ui(
                            &mut light,
                            camera_query.single(),
                            ui,
                            fog.single_mut(),
                        )
                    });
                });
        });
}
