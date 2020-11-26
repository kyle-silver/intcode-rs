use fxhash::FxHashMap;
use crate::*;

enum Value {
    Literal(i64),
    Pointer(i64)
}

trait Arg {
    fn get(&self, rb: i64) -> Value;
}

mod param_mode {
    use super::*;

    struct Immediate {
        val: i64,
    }

    impl Arg for Immediate {
        fn get(&self, _rb: i64) -> Value {
            Value::Literal(self.val)
        }
    }

    struct Position {
        val: i64
    }

    impl Arg for Position {
        fn get(&self, _rb: i64) -> Value {
            Value::Pointer(self.val)
        }
    }

    struct Relative {
        val: i64
    }

    impl Arg for Relative {
        fn get(&self, rb: i64) -> Value {
            Value::Pointer(self.val + rb)
        }
    }
}

enum Action {
    Set { val: i64, addr: i64, },
    SetRb { val: i64, },
    Read { to: i64, },
    Write {  val: i64 },
    Jump { to: i64, },
    Halt
}

mod opcode {
    use super::*;

    trait OpCode {
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
    fn fetch(&self, arg: &Box<dyn Arg>) -> i64 {
        match arg.get(self.rb) {
            Value::Literal(literal) => literal,
            Value::Pointer(address) => self.mem(address),
        }
    }
}

impl IntCodeComputer for PolyIntCode {
    fn run(&mut self) -> State {
        unimplemented!()
    }

    fn out(&self) -> &Vec<i64> {
        unimplemented!()
    }

    fn push(&mut self, val: i64) {
        unimplemented!()
    }

    fn mem(&self, at: i64) -> i64 {
        *self.mem.get(&at).unwrap_or(&0)
    }

    fn state(&self) -> State {
        unimplemented!()
    }
}