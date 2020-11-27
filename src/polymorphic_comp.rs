use fxhash::FxHashMap;
use crate::*;
use std::fmt::Debug;

pub(crate) enum Value {
    Literal(i64),
    Pointer(i64)
}

pub(crate) trait Arg: Debug {
    fn get(&self, rb: i64) -> Value;
    fn arg_clone(&self) -> Box<dyn Arg>;
    fn as_res(&self, rb: i64) -> Box<dyn Arg>;
}

mod param_mode {
    use super::*;

    pub(crate) fn new(modes: i64, val: i64, pos: u32) -> Box<dyn Arg> {
        // get the digit in position `pos` (zero-indexed)
        // i.e. mask(12345, 4) -> `5`
        let mask = (modes / 10i64.pow(pos)) % 10;
        match mask {
            0 => Box::new(Position { val }),
            1 => Box::new(Immediate { val }),
            2 => Box::new(Relative { val }),
            _ => panic!("Unsupported Parameter Mode")
        }
    }

    #[derive(Clone, Debug)]
    struct Immediate {
        val: i64,
    }

    impl Arg for Immediate {
        fn get(&self, _rb: i64) -> Value {
            Value::Literal(self.val)
        }

        fn arg_clone(&self) -> Box<dyn Arg> {
            Box::new(self.clone())
        }

        fn as_res(&self, _rb: i64) -> Box<dyn Arg> {
            self.arg_clone()
        }
    }

    #[derive(Clone, Debug)]
    struct Position {
        val: i64
    }

    impl Arg for Position {
        fn get(&self, _rb: i64) -> Value {
            Value::Pointer(self.val)
        }

        fn arg_clone(&self) -> Box<dyn Arg> {
            Box::new(self.clone())
        }

        fn as_res(&self, _rb: i64) -> Box<dyn Arg> {
            Box::new(Immediate { val: self.val })
        }
    }

    #[derive(Clone, Debug)]
    struct Relative {
        val: i64
    }

    impl Arg for Relative {
        fn get(&self, rb: i64) -> Value {
            Value::Pointer(self.val + rb)
        }

        fn arg_clone(&self) -> Box<dyn Arg> {
            Box::new(self.clone())
        }

        fn as_res(&self, rb: i64) -> Box<dyn Arg> {
            Box::new(Immediate { val: self.val + rb })
//            self.arg_clone()
        }
    }
}

#[derive(Debug)]
pub(crate) enum Action {
    Set { val: i64, addr: i64, },
    SetRb { val: i64, },
    Read { to: i64, },
    Write {  val: i64 },
    Jump { to: i64, },
    Halt
}

pub(crate) trait OpCode: Debug {
    // we *could* pass in a mutable copy of the whole computer, but that would mean exposing
    // a *lot* of internal state. Something about that just smells wrong... you shouldn't
    // be able to get access to the _whole_ system just by implementing this trait.
    // So instead, we need to compromise and let the trait implementor *read* the whole system,
    // but pass back an instruction on how to modify it rather than doing so directly.
    fn execute(&self, comp: &PolyIntCode) -> Action;

    // since we can't directly modify the program counter, we need to have a separate function
    // telling the computer how far to advance.
    fn advance(&self) -> i64 {
        4
    }
}

mod opcode {
    use super::*;

    pub(crate) fn new(data: [i64; 4], rb: i64) -> Box<dyn OpCode> {
        let opcode = data[0] % 100;
        let modes = data[0] / 100;
        let args: Vec<Box<dyn Arg>> = data[1..=3].iter()
            .enumerate()
            .map(|(i, val)| {
                param_mode::new(modes, *val, i as u32)
            })
            .collect();
        match opcode {
            1 => Box::new(Add {
                a: args[0].arg_clone(),
                b: args[1].arg_clone(),
                out: args[2].as_res(rb),
            }),
            2 => Box::new(Mul {
                a: args[0].arg_clone(),
                b: args[1].arg_clone(),
                out: args[2].as_res(rb),
            }),
            3 => Box::new(Read {
                to: args[0].as_res(rb),
            }),
            4 => Box::new(Write {
                val: args[0].arg_clone(),
            }),
            5 => Box::new(JumpIfTrue {
                cond: args[0].arg_clone(),
                to: args[1].arg_clone(),
            }),
            6 => Box::new(JumpIfFalse {
                cond: args[0].arg_clone(),
                to: args[1].arg_clone(),
            }),
            7 => Box::new(LessThan {
                a: args[0].arg_clone(),
                b: args[1].arg_clone(),
                out: args[2].as_res(rb),
            }),
            8 => Box::new(Equals {
                a: args[0].arg_clone(),
                b: args[1].arg_clone(),
                out: args[2].as_res(rb),
            }),
            9 => Box::new(UpdateRb {
                to_add: args[0].arg_clone(),
            }),
            99 => Box::new(Halt {}),
            _ => panic!("Unsupported OpCode")
        }
    }

    #[derive(Debug)]
    struct Add {
        a: Box<dyn Arg>,
        b: Box<dyn Arg>,
        out: Box<dyn Arg>,
    }

    impl OpCode for Add {
        fn execute(&self, comp: &PolyIntCode) -> Action {
            Action::Set {
                val: comp.fetch(&self.a) + comp.fetch(&self.b),
                addr: comp.fetch(&self.out)
            }
        }
    }

    #[derive(Debug)]
    struct Mul {
        a: Box<dyn Arg>,
        b: Box<dyn Arg>,
        out: Box<dyn Arg>,
    }

    impl OpCode for Mul {
        fn execute(&self, comp: &PolyIntCode) -> Action {
            Action::Set {
                val: comp.fetch(&self.a) * comp.fetch(&self.b),
                addr: comp.fetch(&self.out)
            }
        }
    }

    #[derive(Debug)]
    struct Read {
        to: Box<dyn Arg>,
    }

    impl OpCode for Read {
        fn execute(&self, comp: &PolyIntCode) -> Action {
            Action::Read {
                to: comp.fetch(&self.to),
            }
        }

        fn advance(&self) -> i64 {
            2
        }
    }

    #[derive(Debug)]
    struct Write {
        val: Box<dyn Arg>,
    }

    impl OpCode for Write {
        fn execute(&self, comp: &PolyIntCode) -> Action {
            Action::Write {
                val: comp.fetch(&self.val),
            }
        }

        fn advance(&self) -> i64 {
            2
        }
    }

    #[derive(Debug)]
    struct JumpIfTrue {
        cond: Box<dyn Arg>,
        to: Box<dyn Arg>
    }

    impl OpCode for JumpIfTrue {
        fn execute(&self, comp: &PolyIntCode) -> Action {
            Action::Jump {
                to: match comp.fetch(&self.cond) {
                    0 => comp.pc + 3,
                    _ => comp.fetch(&self.to)
                },
            }
        }

        fn advance(&self) -> i64 {
            0
        }
    }

    #[derive(Debug)]
    struct JumpIfFalse {
        cond: Box<dyn Arg>,
        to: Box<dyn Arg>
    }

    impl OpCode for JumpIfFalse {
        fn execute(&self, comp: &PolyIntCode) -> Action {
            Action::Jump {
                to: match comp.fetch(&self.cond) {
                    0 => comp.fetch(&self.to),
                    _ => comp.pc + 3
                },
            }
        }

        fn advance(&self) -> i64 {
            0
        }
    }

    #[derive(Debug)]
    struct LessThan {
        a: Box<dyn Arg>,
        b: Box<dyn Arg>,
        out: Box<dyn Arg>,
    }

    impl OpCode for LessThan {
        fn execute(&self, comp: &PolyIntCode) -> Action {
            Action::Set {
                val: match comp.fetch(&self.a) < comp.fetch(&self.b) {
                    true => 1,
                    false => 0,
                },
                addr: comp.fetch(&self.out),
            }
        }
    }

    #[derive(Debug)]
    struct Equals {
        a: Box<dyn Arg>,
        b: Box<dyn Arg>,
        out: Box<dyn Arg>,
    }

    impl OpCode for Equals {
        fn execute(&self, comp: &PolyIntCode) -> Action {
            Action::Set {
                val: match comp.fetch(&self.a) == comp.fetch(&self.b) {
                    true => 1,
                    false => 0,
                },
                addr: comp.fetch(&self.out),
            }
        }
    }

    #[derive(Debug)]
    struct UpdateRb {
        to_add: Box<dyn Arg>
    }

    impl OpCode for UpdateRb {
        fn execute(&self, comp: &PolyIntCode) -> Action {
            Action::SetRb {
                val: comp.rb + comp.fetch(&self.to_add),
            }
        }

        fn advance(&self) -> i64 {
            2
        }
    }

    #[derive(Debug)]
    struct Halt {}

    impl OpCode for Halt {
        fn execute(&self, _comp: &PolyIntCode) -> Action {
            Action::Halt
        }

        fn advance(&self) -> i64 {
            0
        }
    }
}

#[derive(Debug)]
pub struct PolyIntCode {
    mem: FxHashMap<i64,i64>,
    pc: i64,
    rb: i64,
    inputs: Vec<i64>,
    outputs: Vec<i64>,
}

impl PolyIntCode {
    pub fn new(image: Vec<i64>, inputs: Vec<i64>) -> PolyIntCode {
        let mut mem: FxHashMap<i64,i64> = FxHashMap::default();
        for (k,v) in image.iter().enumerate() {
            mem.insert(k as i64, *v);
        }
        PolyIntCode {
            mem,
            pc: 0,
            rb: 0,
            inputs,
            outputs: Vec::new(),
        }
    }

    fn fetch(&self, arg: &Box<dyn Arg>) -> i64 {
        match arg.get(self.rb) {
            Value::Literal(literal) => literal,
            Value::Pointer(address) => self.mem(address),
        }
    }

    fn set(&mut self, addr: i64, val: i64) {
        self.mem.insert(addr, val);
    }

    fn decode(&self) -> Box<dyn OpCode> {
        let data = [
            self.mem(self.pc),
            self.mem(self.pc + 1),
            self.mem(self.pc + 2),
            self.mem(self.pc + 3),
        ];
        opcode::new(data, self.rb)
    }

    fn execute(&mut self, op: Box<dyn OpCode>) -> State {
        let action = op.execute(self);
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
        self.pc += op.advance();
        State::Running
    }
}

impl IntCodeComputer for PolyIntCode {
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
        match self.decode().execute(self) {
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