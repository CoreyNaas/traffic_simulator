/* 
Intersection "Object"
- Enumeration: IntersectionState
- Structure: Intersection
- Implementation: Intersection
    - new()
    - transition(&mut self)
    - current_state()
*/

pub enum IntersectionState {
    Northsouthgreen,
    Northsouthyellow,
    Allred1,
    Eastwestgreen,
    Eastwestyellow,
    Allred2,
}

pub struct Intersection {
    pub state: IntersectionState,
    pub northbound: crate::traffic_light::TrafficLight,
    pub eastbound: crate::traffic_light::TrafficLight,
    pub westbound: crate::traffic_light::TrafficLight,
    pub southbound: crate::traffic_light::TrafficLight,
}

impl Intersection {
    pub fn new() -> Self {
        Self {
            state: IntersectionState::Allred2,
            northbound: crate::traffic_light::TrafficLight::new(),
            eastbound: crate::traffic_light::TrafficLight::new(),
            westbound: crate::traffic_light::TrafficLight::new(),
            southbound: crate::traffic_light::TrafficLight::new(),
        }
    }

    pub fn transition(&mut self) {
        // Change intersection state
        self.state = match self.state {
            IntersectionState::Northsouthgreen => IntersectionState::Northsouthyellow,
            IntersectionState::Northsouthyellow => IntersectionState::Allred1,
            IntersectionState::Allred1 => IntersectionState::Eastwestgreen,
            IntersectionState::Eastwestgreen => IntersectionState::Eastwestyellow,
            IntersectionState::Eastwestyellow => IntersectionState::Allred2,
            IntersectionState::Allred2 => IntersectionState::Northsouthgreen,
        };

        // Use intersection state to update individual traffic light states
        match self.state {
            IntersectionState::Northsouthgreen => {
                self.northbound.transition(crate::traffic_light::TrafficLightState::Green);
                self.eastbound.transition(crate::traffic_light::TrafficLightState::Red);
                self.westbound.transition(crate::traffic_light::TrafficLightState::Red);
                self.southbound.transition(crate::traffic_light::TrafficLightState::Green);
            },
            IntersectionState::Northsouthyellow =>  {
                self.northbound.transition(crate::traffic_light::TrafficLightState::Yellow);
                self.eastbound.transition(crate::traffic_light::TrafficLightState::Red);
                self.westbound.transition(crate::traffic_light::TrafficLightState::Red);
                self.southbound.transition(crate::traffic_light::TrafficLightState::Yellow);
            },
            IntersectionState::Allred1 =>  {
                self.northbound.transition(crate::traffic_light::TrafficLightState::Red);
                self.eastbound.transition(crate::traffic_light::TrafficLightState::Red);
                self.westbound.transition(crate::traffic_light::TrafficLightState::Red);
                self.southbound.transition(crate::traffic_light::TrafficLightState::Red);
            },
            IntersectionState::Eastwestgreen =>  {
                self.northbound.transition(crate::traffic_light::TrafficLightState::Red);
                self.eastbound.transition(crate::traffic_light::TrafficLightState::Green);
                self.westbound.transition(crate::traffic_light::TrafficLightState::Green);
                self.southbound.transition(crate::traffic_light::TrafficLightState::Red);
            },
            IntersectionState::Eastwestyellow =>  {
                self.northbound.transition(crate::traffic_light::TrafficLightState::Red);
                self.eastbound.transition(crate::traffic_light::TrafficLightState::Yellow);
                self.westbound.transition(crate::traffic_light::TrafficLightState::Yellow);
                self.southbound.transition(crate::traffic_light::TrafficLightState::Red);
            },
            IntersectionState::Allred2 =>  {
                self.northbound.transition(crate::traffic_light::TrafficLightState::Red);
                self.eastbound.transition(crate::traffic_light::TrafficLightState::Red);
                self.westbound.transition(crate::traffic_light::TrafficLightState::Red);
                self.southbound.transition(crate::traffic_light::TrafficLightState::Red);
            },
        };
        //todo!("Add logging function to record date and time transitions occurred");
    }

    pub fn current_state(&self) -> String {
        match self.state {
            IntersectionState::Northsouthgreen =>   "S1: North/South Traffic Green,  East/West Traffic Red".to_owned(),
            IntersectionState::Northsouthyellow =>  "S2: North/South Traffic Yellow, East/West Traffic Red".to_owned(),
            IntersectionState::Allred1 =>           "S3: North/South Traffic Red,    East/West Traffic Red".to_owned(),
            IntersectionState::Eastwestgreen =>     "S4: North/South Traffic Red,    East/West Traffic Green".to_owned(),
            IntersectionState::Eastwestyellow =>    "S5: North/South Traffic Red,    East/West Traffic Yellow".to_owned(),
            IntersectionState::Allred2 =>           "S6: North/South Traffic Red,    East/West Traffic Red".to_owned(),
        }
    }
}