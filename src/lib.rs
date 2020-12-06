pub mod procedural_comp;
pub mod polymorphic_comp;
pub mod poly2_comp;
pub mod poly_client;

#[derive(Debug, PartialEq)]
pub enum State {
    Running,
    Waiting,
    Halted,
}

#[derive(Debug, Clone, Copy)]
pub enum ParamMode {
    Position,
    Immediate,
    Relative,
}

pub trait IntCodeComputer {
    fn run(&mut self) -> State;
    fn out(&self) -> &Vec<i64>;
    fn push(&mut self, val: i64);
    fn mem(&self, at: i64) -> i64;
    fn state(&self) -> State;
}