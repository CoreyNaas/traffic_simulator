// Define the traits for the FSM
pub trait State {
    type Transition;
    fn on_enter(&self) {}
    fn on_exit(&self) {}
    fn on_transition(&self, transition: &Self::Transition) -> Self where Self: Sized;
}

pub struct FiniteStateMachine<S: State> {
    current_state: S,
}

impl<S: State> FiniteStateMachine<S> {
    pub fn new(initial_state: S) -> Self {
        FiniteStateMachine { current_state: initial_state }
    }

    pub fn transition(&mut self, transition: &S::Transition) {
        self.current_state.on_exit();
        self.current_state = self.current_state.on_transition(transition);
        self.current_state.on_enter();
    }

    pub fn current_state(&self) -> &S {
        &self.current_state
    }
}
