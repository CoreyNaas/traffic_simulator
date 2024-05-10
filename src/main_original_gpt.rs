use eframe::{egui, epi};
use crate::egui::CtxRef;

#[derive(PartialEq)]
#[derive(Debug)]
enum TrafficLightState {
    Red,
    Green,
    Yellow,
}

struct TrafficLight {
    state: TrafficLightState,
}

impl TrafficLight {
    fn new() -> Self {
        Self {
            state: TrafficLightState::Red,
        }
    }

    fn transition(&mut self) {
        self.state = match self.state {
            TrafficLightState::Red => TrafficLightState::Green,
            TrafficLightState::Green => TrafficLightState::Yellow,
            TrafficLightState::Yellow => TrafficLightState::Red,
        };
    }

    fn current_color(&self) -> egui::Color32 {
        match self.state {
            TrafficLightState::Red => egui::Color32::RED,
            TrafficLightState::Green => egui::Color32::GREEN,
            TrafficLightState::Yellow => egui::Color32::YELLOW,
        }
    }
}

struct App {
    traffic_light: TrafficLight,
}

impl Default for App {
    fn default() -> Self {
        Self {
            traffic_light: TrafficLight::new(),
        }
    }
}

impl epi::App for App {
    fn name(&self) -> &str {
        "Traffic Light"
    }

    fn update(&mut self, ctx: &CtxRef, _frame: &epi::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Traffic Light Simulator");

            let color = self.traffic_light.current_color();
            
            ui.colored_label(color, format!("{:?}", self.traffic_light.state));

            if ui.button("Next").clicked() {
                self.traffic_light.transition();
            }
        });
    }
}

fn main() {
    let options = eframe::NativeOptions::default();
    eframe::run_native(Box::new(App::default()), options);
}
