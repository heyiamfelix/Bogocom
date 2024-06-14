pub trait RandomGenerator {
    type State: Sized;

    fn from_state(state: Self::State) -> Self
    where
        Self: Sized;

    fn get_state(&self) -> Self::State;
    fn next(&mut self) -> bool;
}
