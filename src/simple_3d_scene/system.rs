use std::f32::consts::PI;

use bevy::prelude::*;
use bevy_egui::{
    egui::{self, Pos2, Ui},
    EguiContexts,
};

use crate::camera_control::CameraController;

use super::BOX_SIZE;

pub fn simple_3d_scene(mut commands: Commands, mut ambient_light: ResMut<AmbientLight>) {
    let camera_controller = CameraController::default();
    let mut camera_transform = Transform::from_xyz(0.0, 0.0, 0.0);
    camera_transform.rotate_x(-20.0 / 180.0 * PI);

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
            transform: Transform::from_xyz(-BOX_SIZE.x * 0.9, BOX_SIZE.y * 1.5, BOX_SIZE.z)
                .looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        camera_controller,
    ));

    ambient_light.color = Color::WHITE;
    ambient_light.brightness = 0.2;
}

pub fn directional_light_ui(light: &mut DirectionalLight, ui: &mut Ui) {
    ui.label("Intensity");
    ui.add(egui::Slider::new(&mut light.illuminance, 100.0..=100_000.0));
    ui.end_row();

    ui.label("Shadows");
    ui.checkbox(&mut light.shadows_enabled, "Enabled");
    ui.end_row();
}

pub fn ui_system(mut light_query: Query<&mut DirectionalLight>, mut contexts: EguiContexts) {
    egui::Window::new("3D world")
        .current_pos(Pos2 { x: 10., y: 60. })
        .show(contexts.ctx_mut(), |ui| {
            egui::Grid::new("3dworld_grid")
                .num_columns(2)
                .spacing([40.0, 4.0])
                .striped(true)
                .show(ui, |ui| {
                    light_query.for_each_mut(|mut light| directional_light_ui(&mut light, ui));
                });
        });
}
