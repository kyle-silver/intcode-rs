use fxhash::FxHashMap;
use std::fmt::Debug;
use crate::*;

/* 
goals:
1. Arg modes will be fixed
2. User-defined opcodes
3. Fixed number of arguments
*/

#[derive(Debug, Copy, Clone)]
pub enum Arg {
    Immediate(i64),
    Position(i64),
    Relative(i64)
}

impl Arg {
    pub fn out(&self, rb: i64) -> Arg {
        match self {
            Arg::Relative(val) => Arg::Immediate(rb + val),
            _ => Arg::Immediate(self.val())
        }
    }

    fn val(&self) -> i64 {
        *match self {
            Arg::Immediate(val) => val,
            Arg::Position(val) => val,
            Arg::Relative(val) => val
        }
    }

    fn new(modes: i64, pos: u32, val: i64) -> Arg {
        // get the digit in position `pos` (zero-indexed)
        // i.e. mask(12345, 4) -> `5`
        let mask = (modes / 10i64.pow(pos)) % 10;
        match mask {
            0 => Arg::Position(val),
            1 => Arg::Immediate(val),
            2 => Arg::Relative(val),
            _ => panic!("Unsupported Parameter Mode")
        }
    }
}

#[derive(Debug)]
pub enum Action {
    Set { val: i64, addr: i64, },
    SetRb { val: i64, },
    Read { to: i64, },
    Write {  val: i64 },
    Jump { to: i64, },
    Halt
}

pub trait OpCode: Debug {
    fn action(&self, comp: &AltPolyIntCode) -> Action;
    fn advance(&self) -> i64;
}

type OpCodeFactory = dyn Fn([Arg; 3]) -> Box<dyn OpCode>;

pub struct AltPolyIntCode {
    pub pc: i64,
    pub rb: i64,
    inputs: Vec<i64>,
    outputs: Vec<i64>,
    mem: FxHashMap<i64,i64>,
    opcodes: FxHashMap<i64, Box<OpCodeFactory>>,
}

impl AltPolyIntCode {
    pub fn new(image: Vec<i64>, inputs: Vec<i64>, opcodes: FxHashMap<i64, Box<OpCodeFactory>>) -> AltPolyIntCode {
        let mut mem: FxHashMap<i64,i64> = FxHashMap::default();
        for (k,v) in image.iter().enumerate() {
            mem.insert(k as i64, *v);
        }
        AltPolyIntCode {
            mem,
            pc: 0,
            rb: 0,
            inputs,
            outputs: Vec::new(),
            opcodes,
        }
    }

    pub fn fetch(&self, arg: &Arg) -> i64 {
        match arg {
            Arg::Immediate(val) => *val,
            Arg::Position(val) => {
                let address = val;
                self.mem(*address)
            },
            Arg::Relative(val) => {
                let offset = val;
                let address = self.rb + offset;
                self.mem(address)
            }
        }
    }

    fn set(&mut self, addr: i64, val: i64) {
        self.mem.insert(addr, val);
    }

    pub fn register(&mut self, code: i64, factory: fn([Arg; 3]) -> Box<dyn OpCode>) {
        // do we care about overriding existing codes? 
        // because this code definitely doesn't
        self.opcodes.insert(code, Box::new(factory));
    }

    fn arglist(&self) -> [Arg; 3] {
        let modes = self.mem(self.pc) / 100;
        [
            Arg::new(modes, 0, self.mem(self.pc+1)),
            Arg::new(modes, 1, self.mem(self.pc+2)),
            Arg::new(modes, 2, self.mem(self.pc+3)),
        ]
    }

    fn decode(&self) -> Box<dyn OpCode> {
        let instruction = self.mem(self.pc) % 100;
        let args = self.arglist();
        // println!("{} -> {}, {:?}", self.pc, instruction, args);
        let factory = self.opcodes.get(&instruction).unwrap();
        factory(args)
    }

    fn execute(&mut self, opcode: Box<dyn OpCode>) -> State {
        let action = opcode.action(&self);
        // println!("Action: {:?}", action);
        match action {
            Action::Set {val, addr} => {
                self.set(addr, val);
            },
            Action::SetRb {val} => {
                self.rb = val;
            },
            Action::Read {to} => {
                if self.inputs.get(0) == None {
                    // don't advance, instruction needs to be replayed
                    return State::Waiting;
                }
                let data = self.inputs.remove(0);
                self.set(to, data);
            },
            Action::Write {val} => {
                self.outputs.push(val);
            },
            Action::Jump {to} => {
                self.pc = to;
            },
            Action::Halt => {
                return State::Halted;
            }
        };
        self.pc += opcode.advance();
        State::Running
    }
}

impl IntCodeComputer for AltPolyIntCode {
    fn run(&mut self) -> State {
        loop {
            let opcode = self.decode();
            let state = self.execute(opcode);
            match state {
                State::Running => continue,
                _ => return state,
            };
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
        match self.decode().action(self) {
            Action::Halt => State::Halted,
            Action::Read {to: _} => {
                match self.inputs.len() {
                    0 => State::Waiting,
                    _ => State::Running,
                }
            }
            _ => State::Running,
        }
    }
}


