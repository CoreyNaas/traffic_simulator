/*
UI "Object"
- Structure: UiApplication
- Implementation: Default for UiApplication
    - default()
- Implementation: eframe::App for UiApplication
    - update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame)
*/

pub struct UiApplication {
    intersection: crate::intersection::Intersection,
}

impl Default for UiApplication {
    fn default() -> Self {
        Self {
            intersection: crate::intersection::Intersection::new(),
        }
    }
}

impl UiApplication {
    pub fn new() -> Self {
        Default::default()
    }
}

impl eframe::App for UiApplication {

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Corey's Traffic Light Simulator");

            // Individual Traffic Lights
            ui.horizontal(|ui| {
                // Northbound lights
                let color = self.intersection.northbound.current_color();
                ui.label("Northbound");
                ui.separator();
                ui.colored_label(color, format!("{:?}", self.intersection.northbound.state));
            });
            ui.horizontal(|ui| {
                // Eastbound lights
                let color = self.intersection.eastbound.current_color();
                ui.label("Eastbound");
                ui.separator();
                ui.colored_label(color, format!("{:?}", self.intersection.eastbound.state));
            });
            ui.horizontal(|ui| {
                // Westbound lights
                let color = self.intersection.westbound.current_color();
                ui.label("Westbound");
                ui.separator();
                ui.colored_label(color, format!("{:?}", self.intersection.westbound.state));
            });
            ui.horizontal(|ui| {
                // Southbound lights
                let color = self.intersection.southbound.current_color();
                ui.label("Southbound");
                ui.separator();
                ui.colored_label(color, format!("{:?}", self.intersection.southbound.state));
            });
            ui.separator();
            
            // Intersection Control
            if ui.button("Next").clicked() {
                self.intersection.transition();
            }
            ui.label("Intersection State: ");
            ui.label(self.intersection.current_state());
        });
    }
}