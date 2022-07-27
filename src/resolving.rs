use std::{collections::HashMap, rc::Rc};

use crate::{
    Ast, AstFile, AstLet, AstProcedure, AstVar, BinaryOperator, BytecodeInstruction,
    BytecodeProcedure, BytecodeProgram, BytecodeValue, ProcedureBody, Type,
};

pub fn resolve_file(file: &AstFile) -> BytecodeProgram {
    let mut procedures = HashMap::new();
    for statement in &file.statements {
        if let Some(procedure) = statement.as_procedure() {
            if let Some(_original) = procedures.insert(procedure.name.clone(), {
                let bytecode_procedure = resolve_procedure(procedure, &procedures);
                let typ = Type::Procedure {
                    parameters: procedure
                        .parameters
                        .iter()
                        .map(|parameter| eval_type(&parameter.typ))
                        .collect(),
                    return_type: Box::new(eval_type(&procedure.return_type)),
                };
                (bytecode_procedure, typ)
            }) {
                todo!("procedure has already been defined")
            }
        } else {
            todo!("non procedure in global scope")
        }
    }
    BytecodeProgram {
        procedures: procedures
            .into_iter()
            .map(|(name, (proc, _))| (name, proc))
            .collect(),
    }
}

fn resolve_procedure(
    procedure: &AstProcedure,
    procedures: &HashMap<String, (BytecodeProcedure, Type)>,
) -> BytecodeProcedure {
    let mut instructions = vec![];
    let mut max_registers = procedure.parameters.len();
    let mut next_register = procedure.parameters.len();
    let mut variables = HashMap::new();

    match &procedure.body {
        ProcedureBody::CompilerGenerated(_) => match &procedure.name as &str {
            "print_int" => {
                instructions.push(BytecodeInstruction::PrintInt { reg: 0 });
                let ret_value = allocate_register(&mut max_registers, &mut next_register);
                instructions.push(BytecodeInstruction::Set {
                    dest: ret_value,
                    value: BytecodeValue::Void,
                });
                instructions.push(BytecodeInstruction::Return { reg: ret_value });
            }

            "println" => {
                instructions.push(BytecodeInstruction::PrintLn);
                let ret_value = allocate_register(&mut max_registers, &mut next_register);
                instructions.push(BytecodeInstruction::Set {
                    dest: ret_value,
                    value: BytecodeValue::Void,
                });
                instructions.push(BytecodeInstruction::Return { reg: ret_value });
            }

            _ => todo!("unknown builtin procedure"),
        },
        ProcedureBody::Scope(scope) => {
            resolve_ast(
                &scope,
                &mut instructions,
                &mut max_registers,
                &mut next_register,
                procedures,
                &mut variables,
            );
            let ret_value = allocate_register(&mut max_registers, &mut next_register);
            instructions.push(BytecodeInstruction::Set {
                dest: ret_value,
                value: BytecodeValue::Void,
            });
            instructions.push(BytecodeInstruction::Return { reg: ret_value });
        }
    }

    BytecodeProcedure {
        instructions,
        max_registers,
    }
}

#[derive(Clone)]
enum Declaration {
    Let(Rc<AstLet>, Type, usize),
    Var(Rc<AstVar>, Type, usize),
}

fn resolve_ast(
    ast: &Ast,
    instructions: &mut Vec<BytecodeInstruction>,
    max_registers: &mut usize,
    next_register: &mut usize,
    procedures: &HashMap<String, (BytecodeProcedure, Type)>,
    variables: &mut HashMap<String, Declaration>,
) -> Option<(usize, Type)> {
    match ast {
        Ast::File(_) => unreachable!(),
        Ast::Procedure(_procedure) => todo!("nested procedures arent supported yet"),
        Ast::Scope(scope) => {
            let mut next_register_copy = *next_register;
            let mut variables_copy = variables.clone();
            for statement in &scope.statements {
                resolve_ast(
                    statement,
                    instructions,
                    max_registers,
                    &mut next_register_copy,
                    procedures,
                    &mut variables_copy,
                );
            }
            None
        }

        Ast::Let(lett) => {
            let reg = allocate_register(max_registers, next_register);

            let decl_typ = if let Some(typ) = &lett.typ {
                Some(eval_type(typ))
            } else {
                None
            };
            let (value_reg, value_typ) = resolve_ast(
                &lett.value,
                instructions,
                max_registers,
                next_register,
                procedures,
                variables,
            )
            .unwrap();
            instructions.push(BytecodeInstruction::Move {
                dest: reg,
                src: value_reg,
            });

            let typ = if let Some(typ) = decl_typ {
                if typ != value_typ {
                    todo!("types do not match")
                }
                typ
            } else {
                value_typ
            };
            if let Some(_original) = variables.insert(
                lett.name.clone(),
                Declaration::Let(lett.clone(), typ.clone(), reg),
            ) {
                todo!("variable was redeclared")
            }

            Some((reg, typ))
        }

        Ast::Var(var) => {
            let reg = allocate_register(max_registers, next_register);

            let decl_typ = if let Some(typ) = &var.typ {
                Some(eval_type(typ))
            } else {
                None
            };
            let (value_reg, value_typ) = resolve_ast(
                &var.value,
                instructions,
                max_registers,
                next_register,
                procedures,
                variables,
            )
            .unwrap();
            instructions.push(BytecodeInstruction::Move {
                dest: reg,
                src: value_reg,
            });

            let typ = if let Some(typ) = decl_typ {
                if typ != value_typ {
                    todo!("types do not match")
                }
                typ
            } else {
                value_typ
            };
            if let Some(_original) = variables.insert(
                var.name.clone(),
                Declaration::Var(var.clone(), typ.clone(), reg),
            ) {
                todo!("variable was redeclared")
            }

            Some((reg, typ))
        }

        Ast::LeftAssign(left_assign) => {
            let (operand, _operand_typ) = resolve_ast(
                &left_assign.operand,
                instructions,
                max_registers,
                next_register,
                procedures,
                variables,
            )
            .unwrap();
            let (value, _value_typ) = resolve_ast(
                &left_assign.value,
                instructions,
                max_registers,
                next_register,
                procedures,
                variables,
            )
            .unwrap();
            // TODO: type checking
            instructions.push(BytecodeInstruction::Move {
                dest: operand,
                src: value,
            });
            None
        }

        Ast::RightAssign(right_assign) => {
            let (value, _value_typ) = resolve_ast(
                &right_assign.value,
                instructions,
                max_registers,
                next_register,
                procedures,
                variables,
            )
            .unwrap();
            let (operand, _operand_typ) = resolve_ast(
                &right_assign.operand,
                instructions,
                max_registers,
                next_register,
                procedures,
                variables,
            )
            .unwrap();
            // TODO: type checking
            instructions.push(BytecodeInstruction::Move {
                dest: operand,
                src: value,
            });
            None
        }

        Ast::If(iff) => {
            let (condition, _condition_typ) = resolve_ast(
                &iff.condition,
                instructions,
                max_registers,
                next_register,
                procedures,
                variables,
            )
            .unwrap();
            // TODO: type checking
            let condition_jump_location = instructions.len();
            instructions.push(BytecodeInstruction::JumpIf {
                location: usize::MAX,
                reg: condition,
            });
            resolve_ast(
                &iff.then_statement,
                instructions,
                max_registers,
                next_register,
                procedures,
                variables,
            );
            let jump_past_else_location = instructions.len();
            instructions.push(BytecodeInstruction::Jump {
                location: usize::MAX,
            });
            *instructions[condition_jump_location]
                .as_jump_if_mut()
                .unwrap()
                .0 = instructions.len();
            if let Some(else_statement) = &iff.else_statement {
                resolve_ast(
                    else_statement,
                    instructions,
                    max_registers,
                    next_register,
                    procedures,
                    variables,
                );
            }
            *instructions[jump_past_else_location].as_jump_mut().unwrap() = instructions.len();
            None
        }

        Ast::While(whilee) => {
            let jump_location = instructions.len();
            let (condition, _condition_typ) = resolve_ast(
                &whilee.condition,
                instructions,
                max_registers,
                next_register,
                procedures,
                variables,
            )
            .unwrap();
            // TODO: type checking
            let condition_jump_location = instructions.len();
            instructions.push(BytecodeInstruction::JumpIf {
                location: usize::MAX,
                reg: condition,
            });
            resolve_ast(
                &whilee.body,
                instructions,
                max_registers,
                next_register,
                procedures,
                variables,
            );
            instructions.push(BytecodeInstruction::Jump {
                location: jump_location,
            });
            *instructions[condition_jump_location]
                .as_jump_if_mut()
                .unwrap()
                .0 = instructions.len();
            None
        }

        Ast::Call(call) => {
            let (operand, operand_typ) = resolve_ast(
                &call.operand,
                instructions,
                max_registers,
                next_register,
                procedures,
                variables,
            )
            .unwrap();
            let mut arguments = vec![];
            // TODO: type checking
            for argument in &call.arguments {
                let (argument_reg, _argument_typ) = resolve_ast(
                    argument,
                    instructions,
                    max_registers,
                    next_register,
                    procedures,
                    variables,
                )
                .unwrap();
                arguments.push(argument_reg);
            }
            let ret = allocate_register(max_registers, next_register);
            instructions.push(BytecodeInstruction::Call {
                proc: operand,
                dest: ret,
                args: arguments,
            });
            Some((ret, *operand_typ.as_procedure().unwrap().1.clone()))
        }

        Ast::Unary(_) => todo!(),

        Ast::Binary(binary) => {
            let (left, left_typ) = resolve_ast(
                &binary.left,
                instructions,
                max_registers,
                next_register,
                procedures,
                variables,
            )
            .unwrap();
            let (right, _right_typ) = resolve_ast(
                &binary.right,
                instructions,
                max_registers,
                next_register,
                procedures,
                variables,
            )
            .unwrap();
            let reg = allocate_register(max_registers, next_register);
            // TODO: type checking
            match binary.operator {
                BinaryOperator::Add => {
                    instructions.push(BytecodeInstruction::Add {
                        dest: reg,
                        a: left,
                        b: right,
                    });
                    Some((reg, left_typ))
                }

                BinaryOperator::Subtract => {
                    instructions.push(BytecodeInstruction::Subtract {
                        dest: reg,
                        a: left,
                        b: right,
                    });
                    Some((reg, left_typ))
                }

                BinaryOperator::Multiply => {
                    instructions.push(BytecodeInstruction::Multiply {
                        dest: reg,
                        a: left,
                        b: right,
                    });
                    Some((reg, left_typ))
                }

                BinaryOperator::Divide => {
                    instructions.push(BytecodeInstruction::Divide {
                        dest: reg,
                        a: left,
                        b: right,
                    });
                    Some((reg, left_typ))
                }

                BinaryOperator::LessThan => {
                    instructions.push(BytecodeInstruction::LessThan {
                        dest: reg,
                        a: left,
                        b: right,
                    });
                    Some((reg, Type::Bool))
                }

                BinaryOperator::GreaterThan => {
                    instructions.push(BytecodeInstruction::GreaterThan {
                        dest: reg,
                        a: left,
                        b: right,
                    });
                    Some((reg, Type::Bool))
                }

                BinaryOperator::LessThanEqual => {
                    instructions.push(BytecodeInstruction::GreaterThan {
                        dest: reg,
                        a: left,
                        b: right,
                    });
                    instructions.push(BytecodeInstruction::LogicalNot { dest: reg, reg });
                    Some((reg, Type::Bool))
                }

                BinaryOperator::GreaterThanEqual => {
                    instructions.push(BytecodeInstruction::LessThan {
                        dest: reg,
                        a: left,
                        b: right,
                    });
                    instructions.push(BytecodeInstruction::LogicalNot { dest: reg, reg });
                    Some((reg, Type::Bool))
                }
            }
        }

        Ast::Name(name) => Some(if let Some(decl) = variables.get(&name.name) {
            match decl {
                Declaration::Let(_, typ, reg) => (*reg, typ.clone()),
                Declaration::Var(_, typ, reg) => (*reg, typ.clone()),
            }
        } else if let Some((_, typ)) = procedures.get(&name.name) {
            let reg = allocate_register(max_registers, next_register);
            instructions.push(BytecodeInstruction::Set {
                dest: reg,
                value: BytecodeValue::Procedure(name.name.clone()),
            });
            (reg, typ.clone())
        } else {
            todo!()
        }),

        Ast::Integer(integer) => {
            let reg = allocate_register(max_registers, next_register);
            if integer.integer > i64::MAX as u128 {
                todo!("integer is too big for int")
            }
            instructions.push(BytecodeInstruction::Set {
                dest: reg,
                value: BytecodeValue::Int(integer.integer as i64),
            });
            Some((reg, Type::Int))
        }
    }
}

fn eval_type(ast: &Ast) -> Type {
    match ast {
        Ast::File(_) => unreachable!(),
        Ast::Procedure(_) => todo!(),
        Ast::Scope(_) => todo!(),
        Ast::Let(_) => todo!(),
        Ast::Var(_) => todo!(),
        Ast::LeftAssign(_) => todo!(),
        Ast::RightAssign(_) => todo!(),
        Ast::If(_) => todo!(),
        Ast::While(_) => todo!(),
        Ast::Call(_) => todo!(),
        Ast::Unary(_) => todo!(),
        Ast::Binary(_) => todo!(),
        Ast::Name(name) => match &name.name as &str {
            "void" => Type::Void,
            "int" => Type::Int,
            "bool" => Type::Bool,
            _ => todo!("unknown type name"),
        },
        Ast::Integer(_) => todo!(),
    }
}

fn allocate_register(max_registers: &mut usize, next_register: &mut usize) -> usize {
    let register = *next_register;
    *next_register += 1;
    if next_register > max_registers {
        *max_registers = *next_register;
    }
    register
}
