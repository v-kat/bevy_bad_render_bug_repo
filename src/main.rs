use bevy::prelude::*;

use bevy::asset::AssetMetaCheck;
use bevy_egui::{egui, EguiContexts, EguiPlugin};
use egui::{CentralPanel, Color32};

pub const UV_RECT: egui::Rect =
    egui::Rect::from_min_max(egui::pos2(0.0, 0.0), egui::pos2(1.0, 1.0));

fn main() {
    App::new()
        .insert_resource(AssetMetaCheck::Never)
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Test".to_string(),
                    ..default()
                }),
                ..default()
            }),
            EguiPlugin,
        ))
        .add_systems(Update, home_menu_system)
        .run();

}

fn home_menu_system(
    mut contexts: EguiContexts,
    asset_server: Res<AssetServer>,
) {
    let img_handle = asset_server.load("tester.webp");
    let background_texture_id = contexts.add_image(img_handle);
    let screen_rect = contexts.ctx_mut().screen_rect();

    CentralPanel::default().show(contexts.ctx_mut(), |ui| {
        ui.vertical_centered(|ui| {
            ui.painter()
                .image(background_texture_id, screen_rect, UV_RECT, Color32::WHITE);
            ui.add_space(screen_rect.max.y * 0.5);
            if ui.button("Test").clicked() {
            }
            ui.add_space(screen_rect.max.y * 0.025);
            if ui.button("Screen").clicked() {
            };
        });
    });
}
