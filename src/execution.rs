use std::io::Write;

use crate::{BytecodeInstruction, BytecodeProcedure, BytecodeProgram, BytecodeValue};

pub fn execute_program(program: &BytecodeProgram, f: &mut dyn Write) {
    execute_procedure(
        program,
        &program.procedures[program.main_proc_index],
        &[],
        f,
    );
}

pub fn execute_procedure(
    program: &BytecodeProgram,
    procedure: &BytecodeProcedure,
    args: &[BytecodeValue],
    f: &mut dyn Write,
) -> BytecodeValue {
    let mut ip = 0;
    let mut registers = vec![BytecodeValue::Void; procedure.max_registers];
    for (i, arg) in args.iter().enumerate() {
        registers[i] = arg.clone();
    }
    loop {
        match &procedure.instructions[ip] {
            BytecodeInstruction::Set { dest, value } => registers[*dest] = value.clone(),
            BytecodeInstruction::Move { dest, src } => registers[*dest] = registers[*src].clone(),
            BytecodeInstruction::Add { dest, a, b } => {
                registers[*dest] = BytecodeValue::Int(
                    registers[*a].as_int().unwrap() + registers[*b].as_int().unwrap(),
                )
            }
            BytecodeInstruction::Subtract { dest, a, b } => {
                registers[*dest] = BytecodeValue::Int(
                    registers[*a].as_int().unwrap() - registers[*b].as_int().unwrap(),
                )
            }
            BytecodeInstruction::Multiply { dest, a, b } => {
                registers[*dest] = BytecodeValue::Int(
                    registers[*a].as_int().unwrap() * registers[*b].as_int().unwrap(),
                )
            }
            BytecodeInstruction::Divide { dest, a, b } => {
                registers[*dest] = BytecodeValue::Int(
                    registers[*a].as_int().unwrap() / registers[*b].as_int().unwrap(),
                )
            }
            BytecodeInstruction::LessThan { dest, a, b } => {
                registers[*dest] = BytecodeValue::Bool(
                    registers[*a].as_int().unwrap() < registers[*b].as_int().unwrap(),
                )
            }
            BytecodeInstruction::GreaterThan { dest, a, b } => {
                registers[*dest] = BytecodeValue::Bool(
                    registers[*a].as_int().unwrap() > registers[*b].as_int().unwrap(),
                )
            }
            BytecodeInstruction::Negate { dest, reg } => {
                registers[*dest] = BytecodeValue::Int(-registers[*reg].as_int().unwrap())
            }
            BytecodeInstruction::LogicalNot { dest, reg } => {
                registers[*dest] = BytecodeValue::Bool(!registers[*reg].as_bool().unwrap())
            }
            BytecodeInstruction::Jump { location } => {
                ip = *location;
                continue;
            }
            BytecodeInstruction::JumpIf { location, reg } => {
                if *registers[*reg].as_bool().unwrap() {
                    ip = *location;
                    continue;
                }
            }
            BytecodeInstruction::Call { proc, dest, args } => {
                registers[*dest] = execute_procedure(
                    program,
                    &program.procedures[*registers[*proc].as_procedure().unwrap()],
                    &args
                        .iter()
                        .map(|reg| registers[*reg].clone())
                        .collect::<Vec<_>>(),
                    f,
                )
            }
            BytecodeInstruction::Return { reg } => return registers[*reg].clone(),
            BytecodeInstruction::PrintInt { reg } => {
                write!(f, "{}", registers[*reg].as_int().unwrap()).unwrap()
            }
            BytecodeInstruction::PrintLn => writeln!(f).unwrap(),
        }
        ip += 1;
    }
}
