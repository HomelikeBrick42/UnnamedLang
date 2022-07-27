use std::collections::HashMap;

use derive_more::Display;
use enum_as_inner::EnumAsInner;

#[derive(Clone, Debug)]
pub struct BytecodeProgram {
    pub procedures: HashMap<String, BytecodeProcedure>,
}

#[derive(Clone, Debug)]
pub struct BytecodeProcedure {
    pub instructions: Vec<BytecodeInstruction>,
    pub max_registers: usize,
}

#[derive(Clone, Debug, Display, EnumAsInner)]
pub enum BytecodeInstruction {
    #[display(fmt = "set {}, {}", dest, value)]
    Set { dest: usize, value: BytecodeValue },
    #[display(fmt = "mov {}, {}", dest, src)]
    Move { dest: usize, src: usize },
    #[display(fmt = "add {}, {}, {}", dest, a, b)]
    Add { dest: usize, a: usize, b: usize },
    #[display(fmt = "sub {}, {}, {}", dest, a, b)]
    Subtract { dest: usize, a: usize, b: usize },
    #[display(fmt = "mul {}, {}, {}", dest, a, b)]
    Multiply { dest: usize, a: usize, b: usize },
    #[display(fmt = "div {}, {}, {}", dest, a, b)]
    Divide { dest: usize, a: usize, b: usize },
    #[display(fmt = "lt {}, {}, {}", dest, a, b)]
    LessThan { dest: usize, a: usize, b: usize },
    #[display(fmt = "gt {}, {}, {}", dest, a, b)]
    GreaterThan { dest: usize, a: usize, b: usize },
    #[display(fmt = "not {}, {}", dest, reg)]
    LogicalNot { dest: usize, reg: usize },
    #[display(fmt = "jmp {}", location)]
    Jump { location: usize },
    #[display(fmt = "jnz {}, {}", location, reg)]
    JumpIf { location: usize, reg: usize },
    #[display(fmt = "call {}, {}, {:?}", proc, dest, args)]
    Call {
        proc: usize,
        dest: usize,
        args: Vec<usize>,
    },
    #[display(fmt = "ret {}", reg)]
    Return { reg: usize },
    #[display(fmt = "print_int {}", reg)]
    PrintInt { reg: usize },
    #[display(fmt = "println")]
    PrintLn,
}

#[derive(Clone, Debug, Display, EnumAsInner)]
pub enum BytecodeValue {
    #[display(fmt = "void")]
    Void,
    #[display(fmt = "{}", _0)]
    Int(i64),
    #[display(fmt = "{}", _0)]
    Bool(bool),
    #[display(fmt = "proc {}", _0)]
    Procedure(String),
}
