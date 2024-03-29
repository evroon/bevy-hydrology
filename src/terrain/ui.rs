use bevy::{
    asset::{Assets, Handle},
    ecs::{
        entity::Entity,
        system::{Query, ResMut},
    },
    render::mesh::Mesh,
};
use bevy_egui::{
    egui::{self, Pos2, Ui},
    EguiContexts,
};

use super::{
    hydrology::{apply_hydrology, HydrologyConfig},
    system::{rebuild_terrain, TerrainBuildConfig},
};

pub fn terrain_ui(
    meshes: ResMut<Assets<Mesh>>,
    mut terrain_query: Query<(Entity, &Handle<Mesh>, &mut TerrainBuildConfig)>,
    ui: &mut Ui,
) {
    ui.add(egui::Slider::new(&mut terrain_query.single_mut().2.seed, 0..=120).text("Seed"));
    ui.end_row();
    ui.add(
        egui::Slider::new(
            &mut terrain_query.single_mut().2.base_amplitude,
            0.0..=120.0,
        )
        .text("Base amplitude"),
    );
    ui.end_row();
    ui.add(
        egui::Slider::new(
            &mut terrain_query.single_mut().2.base_frequency,
            0.0005..=0.05,
        )
        .text("Base frequency"),
    );
    ui.end_row();

    if ui.button("Rebuild terrain").clicked() {
        rebuild_terrain(meshes, terrain_query);
    };
    ui.end_row();
}

pub fn hydrology_ui(
    mut hydrology_query: Query<(Entity, &Handle<Mesh>, &mut HydrologyConfig)>,
    ui: &mut Ui,
) {
    let config = &mut hydrology_query.single_mut().2;
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
        egui::Slider::new(&mut config.drops_per_frame_per_chunck, 0..=2048).text("Drops per frame"),
    );
    ui.end_row();
    ui.add(egui::Slider::new(&mut config.min_volume, 0.001..=0.1).text("Minimum volume"));
    ui.end_row();

    if ui.button("Reset to defaults").clicked() {
        let default = HydrologyConfig::default();
        config.dt = default.dt;
        config.density = default.density;
        config.evap_rate = default.evap_rate;
        config.deposition_rate = default.deposition_rate;
        config.friction = default.friction;
        config.min_volume = default.min_volume;
        config.drops_per_frame_per_chunck = default.drops_per_frame_per_chunck;
    };
}

pub fn ui_system(
    mut meshes: ResMut<Assets<Mesh>>,
    terrain_query: Query<(Entity, &Handle<Mesh>, &mut TerrainBuildConfig)>,
    hydrology_query: Query<(Entity, &Handle<Mesh>, &mut HydrologyConfig)>,
    mut contexts: EguiContexts,
) {
    meshes
        .iter_mut()
        .for_each(|x| apply_hydrology(x.1, hydrology_query.single().2));

    egui::Window::new("Terrain Generation")
        .current_pos(Pos2 { x: 10., y: 160. })
        .show(contexts.ctx_mut(), |ui| {
            egui::Grid::new("3dworld_grid")
                .num_columns(2)
                .spacing([40.0, 4.0])
                .striped(true)
                .show(ui, |ui| {
                    terrain_ui(meshes, terrain_query, ui);
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
                    hydrology_ui(hydrology_query, ui);
                });
        });
}
