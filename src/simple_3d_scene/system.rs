use std::f32::consts::PI;

use bevy::prelude::*;
use bevy_egui::{
    egui::{self, Pos2, Ui},
    EguiContexts,
};
use bevy_panorbit_camera::PanOrbitCamera;

pub fn simple_3d_scene(mut commands: Commands, mut ambient_light: ResMut<AmbientLight>) {
    let mut camera_transform = Transform::from_xyz(0.0, 0.0, 0.0);
    camera_transform.rotate_x(-30.0 / 180.0 * PI);

    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: 22000.0,
            shadows_enabled: true,
            ..default()
        },
        transform: camera_transform,
        ..default()
    });
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(-240.0, 240.0, 000.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        PanOrbitCamera {
            button_pan: MouseButton::Middle,
            button_orbit: MouseButton::Left,
            ..Default::default()
        },
    ));

    ambient_light.color = Color::WHITE;
    ambient_light.brightness = 0.4;
}

pub fn directional_light_ui(
    light: &mut DirectionalLight,
    camera_transform: (&Transform, &Camera),
    ui: &mut Ui,
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
}

pub fn ui_system(
    mut light_query: Query<&mut DirectionalLight>,
    camera_query: Query<(&Transform, &Camera)>,
    mut contexts: EguiContexts,
) {
    egui::Window::new("3D world")
        .current_pos(Pos2 { x: 10., y: 10. })
        .show(contexts.ctx_mut(), |ui| {
            egui::Grid::new("3dworld_grid")
                .num_columns(2)
                .spacing([40.0, 4.0])
                .striped(true)
                .show(ui, |ui| {
                    light_query.for_each_mut(|mut light| {
                        directional_light_ui(&mut light, camera_query.single(), ui)
                    });
                });
        });
}
