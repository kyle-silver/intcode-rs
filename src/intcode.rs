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
}