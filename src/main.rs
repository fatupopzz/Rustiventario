mod models;
mod inventario;
mod utils;
mod ui;

use ui::app::InventarioApp;

fn main() {
    let native_options = eframe::NativeOptions {
        viewport: eframe::egui::ViewportBuilder::default()
            .with_inner_size(egui::vec2(1024.0, 768.0))
            .with_min_inner_size(egui::vec2(800.0, 600.0)),
        ..Default::default()
    };
    
    eframe::run_native(
        "Sistema de Gestión de Inventario",
        native_options,
        Box::new(|cc| Box::new(InventarioApp::new(cc))),
    ).unwrap();
}
