use fxhash::FxHashMap;
use crate::poly2_comp::{
    Action, 
    AltPolyIntCode, 
    Arg, 
    OpCode
};

#[derive(Debug)]
struct Add {
    a: Arg,
    b: Arg,
    out: Arg,
}

impl OpCode for Add {
    fn action(&self, comp: &AltPolyIntCode) -> Action {
        Action::Set {
            val: comp.fetch(&self.a) + comp.fetch(&self.b),
            addr: comp.fetch(&self.out.out(comp.rb))
        }
    }

    fn advance(&self) -> i64 {
        4
    }
}

#[derive(Debug)]
struct Mul {
    a: Arg,
    b: Arg,
    out: Arg,
}

impl OpCode for Mul {
    fn action(&self, comp: &AltPolyIntCode) -> Action {
        Action::Set {
            val: comp.fetch(&self.a) * comp.fetch(&self.b),
            addr: comp.fetch(&self.out.out(comp.rb))
        }
    }

    fn advance(&self) -> i64 {
        4
    }
}

#[derive(Debug)]
struct Read {
    to: Arg,
}

impl OpCode for Read {
    fn action(&self, comp: &AltPolyIntCode) -> Action {
        Action::Read {
            to: comp.fetch(&self.to.out(comp.rb)),
        }
    }

    fn advance(&self) -> i64 {
        2
    }
}

#[derive(Debug)]
pub struct Write {
    val: Arg,
}

impl OpCode for Write {
    fn action(&self, comp: &AltPolyIntCode) -> Action {
        Action::Write {
            val: comp.fetch(&self.val),
        }
    }

    fn advance(&self) -> i64 {
        2
    }
}

#[derive(Debug)]
pub struct JumpIfTrue {
    cond: Arg,
    to: Arg,
}

impl OpCode for JumpIfTrue {
    fn action(&self, comp: &AltPolyIntCode) -> Action {
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
pub struct JumpIfFalse {
    cond: Arg,
    to: Arg
}

impl OpCode for JumpIfFalse {
    fn action(&self, comp: &AltPolyIntCode) -> Action {
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
pub struct LessThan {
    pub a: Arg,
    pub b: Arg,
    pub out: Arg,
}

impl OpCode for LessThan {
    fn action(&self, comp: &AltPolyIntCode) -> Action {
        Action::Set {
            val: match comp.fetch(&self.a) < comp.fetch(&self.b) {
                true => 1,
                false => 0,
            },
            addr: comp.fetch(&self.out.out(comp.rb)),
        }
    }

    fn advance(&self) -> i64 {
        4
    }
}

#[derive(Debug)]
struct Equals {
    a: Arg,
    b: Arg,
    out: Arg,
}

impl OpCode for Equals {
    fn action(&self, comp: &AltPolyIntCode) -> Action {
        Action::Set {
            val: match comp.fetch(&self.a) == comp.fetch(&self.b) {
                true => 1,
                false => 0,
            },
            addr: comp.fetch(&self.out.out(comp.rb)),
        }
    }

    fn advance(&self) -> i64 {
        4
    }
}

#[derive(Debug)]
struct UpdateRb {
    to_add: Arg,
}

impl OpCode for UpdateRb {
    fn action(&self, comp: &AltPolyIntCode) -> Action {
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
    fn action(&self, _comp: &AltPolyIntCode) -> Action {
        Action::Halt
    }

    fn advance(&self) -> i64 {
        0
    }
}

pub fn client_comp(image: Vec<i64>, inputs: Vec<i64>) -> AltPolyIntCode {
    let mut comp = AltPolyIntCode::new(image, inputs, FxHashMap::default());
    comp.register(1, |args| Box::new(Add {
        a: args[0],
        b: args[1],
        out: args[2],
    }));
    comp.register(2, |args| Box::new(Mul {
        a: args[0],
        b: args[1],
        out: args[2],
    }));
    comp.register(3, |args| Box::new(Read {
        to: args[0],
    }));
    comp.register(4, |args| Box::new(Write {
        val: args[0],
    }));
    comp.register(5, |args| Box::new(JumpIfTrue {
        cond: args[0],
        to: args[1],
    }));
    comp.register(6, |args| Box::new(JumpIfFalse {
        cond: args[0],
        to: args[1],
    }));
    comp.register(7, |args| Box::new(LessThan {
        a: args[0],
        b: args[1],
        out: args[2],
    }));
    comp.register(8, |args| Box::new(Equals {
        a: args[0],
        b: args[1],
        out: args[2],
    }));
    comp.register(9, |args| Box::new(UpdateRb {
        to_add: args[0]
    }));
    comp.register(99, |_args| Box::new(Halt{}));
    comp
}