use fxhash::FxHashMap;
use crate::*;

#[derive(Debug, Clone, Copy)]
pub struct Arg {
    val: i64,
    mode: ParamMode
}

#[derive(Debug)]
enum OpCode {
    Add { a: Arg, b: Arg, out: Arg },
    Mul { a: Arg, b: Arg, out: Arg },
    Read { to: Arg },
    Write { val: Arg },
    // jump to jaddr if cond is nonzero
    JumpIfTrue { cond: Arg, to: Arg },
    // jump to jaddr if cond is zero
    JumpIfFalse { cond: Arg, to: Arg },
    // writes 1 to resloc if arg1 < arg2, else 0
    LessThan { a: Arg, b: Arg, out: Arg },
    // writes 1 to resloc if arg1 == arg2, else 0
    Equals { a: Arg, b: Arg, out: Arg },
    // add value to IntCodeComputer.rb
    UpdateRb { val: Arg },
    Halt,
}

impl OpCode {
    fn new(data: [i64; 4]) -> OpCode {
        let opcode = data[0] % 100;
        let modes = data[0] / 100;
        let args: Vec<Arg> = data[1..=3].iter()
            .enumerate()
            .map(|(i, val)| {
                Arg {
                    val: *val,
                    mode: OpCode::param_mode(modes, i as u32)
                }
            })
            .collect();
        match opcode {
            1 => OpCode::Add { a: args[0], b: args[1], out: args[2] },
            2 => OpCode::Mul { a: args[0], b: args[1], out: args[2] },
            3 => OpCode::Read { to: args[0] },
            4 => OpCode::Write { val: args[0] },
            5 => OpCode::JumpIfTrue { cond: args[0], to: args[1] },
            6 => OpCode::JumpIfFalse { cond: args[0], to: args[1] },
            7 => OpCode::LessThan { a: args[0], b: args[1], out: args[2] },
            8 => OpCode::Equals { a: args[0], b: args[1], out: args[2] },
            9 => OpCode::UpdateRb { val: args[0] },
            99 => OpCode::Halt,
            _ => panic!("Unsupported OpCode")
        }
    }

    fn param_mode(val: i64, pos: u32) -> ParamMode {
        // get the digit in position `pos` (zero-indexed)
        // i.e. mask(12345, 4) -> `5`
        let mask = (val / 10i64.pow(pos)) % 10;
        match mask {
            0 => ParamMode::Position,
            1 => ParamMode::Immediate,
            2 => ParamMode::Relative,
            _ => panic!("Unsupported Parameter Mode")
        }
    }
}

#[derive(Debug)]
pub struct ProcIntCode {
    mem: FxHashMap<i64,i64>,
    pc: i64,
    rb: i64,
    inputs: Vec<i64>,
    outputs: Vec<i64>,
}

impl ProcIntCode {
    pub fn new(image: Vec<i64>, inputs: Vec<i64>) -> ProcIntCode {
        let mut mem: FxHashMap<i64,i64> = FxHashMap::default();
        for (k,v) in image.iter().enumerate() {
            mem.insert(k as i64, *v);
        }
        ProcIntCode {
            mem,
            pc: 0,
            rb: 0,
            inputs,
            outputs: Vec::new(),
        }
    }

    fn set(&mut  self, arg: Arg, val: i64) {
        let base = match arg.mode {
            ParamMode::Relative => self.rb,
            _ => 0,
        };
        let address = base as i64 + arg.val;
        self.mem.insert(address, val);
    }

    fn fetch(&self, arg: Arg) -> i64 {
        match arg.mode {
            ParamMode::Immediate => arg.val,
            ParamMode::Position => {
                let address = arg.val;
                self.mem(address)
            },
            ParamMode::Relative => {
                let offset = arg.val;
                let address = self.rb + offset;
                self.mem(address)
            }
        }
    }

    fn decode(&self) -> OpCode {
        let data = [
            self.mem(self.pc),
            self.mem(self.pc + 1),
            self.mem(self.pc + 2),
            self.mem(self.pc + 3),
        ];
        OpCode::new(data)
    }

    fn execute(&mut self, opcode: OpCode) -> State {
        match opcode {
            OpCode::Add {a, b, out} => {
                let val = self.fetch(a) + self.fetch(b);
                self.set(out, val);
                self.pc += 4;
            },
            OpCode::Mul {a, b, out} => {
                let val = self.fetch(a) * self.fetch(b);
                self.set(out, val);
                self.pc += 4;
            },
            OpCode::Read {to} => {
                if self.inputs.get(0) == None {
                    return State::Waiting;
                }
                let data = self.inputs.get(0).unwrap().clone();
                self.inputs.remove(0);
                self.set(to, data);
                self.pc += 2;
            },
            OpCode::Write { val } => {
                self.outputs.push(self.fetch(val));
                self.pc += 2;
            },
            OpCode::JumpIfTrue {cond, to} => {
                if self.fetch(cond) != 0 {
                    self.pc = self.fetch(to);
                } else {
                    self.pc += 3;
                }
            }
            OpCode::JumpIfFalse {cond, to} => {
                if self.fetch(cond) == 0 {
                    self.pc = self.fetch(to);
                } else {
                    self.pc += 3;
                }
            },
            OpCode::LessThan {a, b, out} => {
                let res = match self.fetch(a) < self.fetch(b) {
                    true => 1,
                    false => 0,
                };
                self.set(out, res);
                self.pc += 4;
            },
            OpCode::Equals {a, b, out} => {
                let res = match self.fetch(a) == self.fetch(b) {
                    true => 1,
                    false => 0,
                };
                self.set(out, res);
                self.pc += 4;
            },
            OpCode::UpdateRb {val} => {
                let rb = self.rb as i64;
                let updated_rb = rb + self.fetch(val);
                self.rb = updated_rb;
                self.pc += 2;
            }
            OpCode::Halt => {
                return State::Halted
            }
        }
        State::Running
    }
}

impl IntCodeComputer for ProcIntCode {
    fn run(&mut self) -> State {
        loop {
            let opcode = self.decode();
            let state = self.execute(opcode);
            if let State::Running = state {
                continue;
            } else {
                return state;
            }
        }
    }

    fn out(&self) -> &Vec<i64> {
        &self.outputs
    }

    fn push(&mut self, val: i64) {
        self.inputs.push(val)
    }

    fn mem(&self, at: i64) -> i64 {
        *self.mem.get(&at).unwrap_or(&0)
    }

    fn state(&self) -> State {
        match self.decode() {
            OpCode::Halt => State::Halted,
            OpCode::Read {to: _} => {
                match self.inputs.len() {
                    0 => State::Waiting,
                    _ => State::Running,
                }
            }
            _ => State::Running,
        }
    }
}