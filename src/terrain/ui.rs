use bevy::ecs::system::ResMut;
use bevy_egui::{
    egui::{self, Pos2, Ui},
    EguiContexts,
};

use super::{hydrology_compute::HydrologyConfig, TerrainBuildConfig};

pub fn terrain_ui(config: &mut TerrainBuildConfig, ui: &mut Ui) {
    ui.add(egui::Slider::new(&mut config.seed, 0..=120).text("Seed"));
    ui.end_row();
    ui.add(egui::Slider::new(&mut config.base_amplitude, 0.0..=120.0).text("Base amplitude"));
    ui.end_row();
    ui.add(egui::Slider::new(&mut config.base_frequency, 0.0005..=0.05).text("Base frequency"));
    ui.end_row();

    if ui.button("Rebuild terrain").clicked() {};
    ui.end_row();
}

pub fn hydrology_ui(config: &mut HydrologyConfig, ui: &mut Ui) {
    ui.add(egui::Slider::new(&mut config.dt, 0.01..=2.0).text("dt"));
    ui.end_row();
    ui.add(egui::Slider::new(&mut config.density, 0.1..=3.0).text("Density"));
    ui.end_row();
    ui.add(egui::Slider::new(&mut config.deposition_rate, 0.01..=1.0).text("Deposition Rate"));
    ui.end_row();
    ui.add(egui::Slider::new(&mut config.evap_rate, 0.0001..=0.01).text("Evaporation Rate"));
    ui.end_row();
    ui.add(egui::Slider::new(&mut config.friction, 0.5..=0.005).text("Friction"));
    ui.end_row();
    ui.add(
        egui::Slider::new(&mut config.drops_per_frame_per_chunk, 0..=2048).text("Drops per frame"),
    );
    ui.end_row();
    ui.add(egui::Slider::new(&mut config.min_volume, 0.001..=0.1).text("Minimum volume"));
    ui.end_row();
    ui.add(egui::Slider::new(&mut config.max_drops, 0..=400_000).text("Maximum drops"));
    ui.end_row();
    ui.label("Drops count: ".to_owned() + &config.drop_count.to_string());
    ui.end_row();

    if ui.button("Reset to defaults").clicked() {
        let default = HydrologyConfig::default();
        config.dt = default.dt;
        config.density = default.density;
        config.evap_rate = default.evap_rate;
        config.deposition_rate = default.deposition_rate;
        config.friction = default.friction;
        config.min_volume = default.min_volume;
        config.drops_per_frame_per_chunk = default.drops_per_frame_per_chunk;
        config.drop_count = default.drop_count;
        config.max_drops = default.max_drops;
    };
}

pub fn ui_system(
    mut terrain_uniform_config: ResMut<TerrainBuildConfig>,
    mut hydrology_config: ResMut<HydrologyConfig>,
    mut contexts: EguiContexts,
) {
    egui::Window::new("Terrain Generation")
        .current_pos(Pos2 { x: 10., y: 190. })
        .show(contexts.ctx_mut(), |ui| {
            egui::Grid::new("3dworld_grid")
                .num_columns(2)
                .spacing([40.0, 4.0])
                .striped(true)
                .show(ui, |ui| {
                    terrain_ui(terrain_uniform_config.as_mut(), ui);
                });
        });

    egui::Window::new("Hydrology")
        .current_pos(Pos2 { x: 10., y: 320. })
        .show(contexts.ctx_mut(), |ui| {
            egui::Grid::new("3dworld_grid")
                .num_columns(2)
                .spacing([40.0, 4.0])
                .striped(true)
                .show(ui, |ui| {
                    hydrology_ui(hydrology_config.as_mut(), ui);
                });
        });
}
