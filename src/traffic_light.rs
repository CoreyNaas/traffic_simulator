
/*
Traffic Light "Object"
- Enumeration: TrafficLightState
- Structure: TrafficLight
- Implementation: TrafficLight
    - new()
    - transition(&mut self, nextstate: TrafficLightState)
    - current_color()
*/

#[derive(PartialEq)]
#[derive(Debug)]
pub enum TrafficLightState {
    Red,
    Green,
    Yellow,
}

// Implementation for the traffic light FSM
#[derive(Debug)]
pub enum TrafficLightTransition {
    Change,
}

impl crate::fsm::State for TrafficLightState {
    type Transition = TrafficLightTransition;

    fn on_transition(&self, transition: &Self::Transition) -> Self {
        match (self, transition) {
            (TrafficLightState::Red, TrafficLightTransition::Change) => TrafficLightState::Green,
            (TrafficLightState::Green, TrafficLightTransition::Change) => TrafficLightState::Yellow,
            (TrafficLightState::Yellow, TrafficLightTransition::Change) => TrafficLightState::Red,
        }
    }

    fn on_enter(&self) {}
    
    fn on_exit(&self) {}
    
    // Optionally implement on_enter and on_exit if needed
}


pub struct TrafficLight {
    pub state: TrafficLightState,
}

impl TrafficLight {
    pub fn new() -> Self {
        Self {
            state: TrafficLightState::Red,
        }
    }

    pub fn transition(&mut self, nextstate: TrafficLightState) {
        self.state = nextstate;
    }

    pub fn current_color(&self) -> egui::Color32 {
        match self.state {
            TrafficLightState::Red => egui::Color32::RED,
            TrafficLightState::Green => egui::Color32::GREEN,
            TrafficLightState::Yellow => egui::Color32::YELLOW,
        }
    }
}