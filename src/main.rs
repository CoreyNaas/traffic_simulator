#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod traffic_light;
mod intersection;
mod ui;
mod fsm;

fn main() {
    // define native options for the eframe
    let native_options: eframe::NativeOptions = eframe::NativeOptions{
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([400.0, 300.0])
            .with_min_inner_size([330.0, 220.0])
            .with_icon(
                eframe::icon_data::from_png_bytes(&include_bytes!("../assets/icon-256.png")[..]).expect("Failed to loan icon."),
            ),
        ..Default::default()
    };
    
    // create the eframe with the native_options, and create a new Box with the UiApplication
    let _ = eframe::run_native(
        "Corey's Traffic Signal Simulator", 
        native_options, 
        Box::new(|_cc| Box::new(crate::ui::UiApplication::new())));
}
