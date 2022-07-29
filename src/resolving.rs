use std::{collections::HashMap, rc::Rc};

use derive_more::Display;
use enum_as_inner::EnumAsInner;

use crate::{
    Ast, AstFile, AstLet, AstProcedure, AstVar, BinaryOperator, BytecodeInstruction,
    BytecodeProcedure, BytecodeProgram, BytecodeValue, Parameter, ProcedureBody, SourceSpan, Type,
    UnaryOperator,
};

#[derive(Clone, PartialEq, Debug, Display, EnumAsInner)]
pub enum ResolvingError {
    #[display(
        fmt = "{}: '{}' has already been declared at {}",
        new_declaration,
        name,
        old_declaration
    )]
    Redeclaration {
        name: String,
        old_declaration: SourceSpan,
        new_declaration: SourceSpan,
    },
    #[display(fmt = "{}: '{}' has not been declared", name_location, name)]
    UndeclaredName {
        name: String,
        name_location: SourceSpan,
    },
    #[display(fmt = "{}: Type '{}' has not been declared", name_location, name)]
    UndeclaredType {
        name: String,
        name_location: SourceSpan,
    },
    #[display(fmt = "{}: Expected a type", location)]
    ExpectedType { location: SourceSpan },
    #[display(
        fmt = "{}: Integer '{}' is too big for type '{}'",
        integer_location,
        value,
        typ
    )]
    IntegerTooBigForType {
        value: u128,
        typ: Type,
        integer_location: SourceSpan,
    },
    #[display(
        fmt = "{}: Expected type '{}', but got type '{}'",
        location,
        expected_typ,
        typ
    )]
    ExpectedTypeButGotType {
        location: SourceSpan,
        expected_typ: Type,
        typ: Type,
    },
    #[display(
        fmt = "{}: Expected a procedure type, but got type '{}'",
        location,
        typ
    )]
    ExpectedProcedure { location: SourceSpan, typ: Type },
    #[display(fmt = "{}: Only procedures are allowed at global scope", location)]
    NonProcedureAtGlobalScope { location: SourceSpan },
    #[display(fmt = "{}: Unknown #compiler procedure '{}'", location, name)]
    UnknownCompilerProcedure { location: SourceSpan, name: String },
    #[display(fmt = "{}: expression is not assignable", location)]
    NotAssignable { location: SourceSpan },
    #[display(fmt = "Unable to find main procedure")]
    NoMainProcedure,
    #[display(fmt = "{}: Not all control paths return a value", location)]
    NoReturn { location: SourceSpan },
}

pub fn resolve_file(file: &AstFile) -> Result<BytecodeProgram, ResolvingError> {
    let mut variables = HashMap::new();
    let mut procedures = vec![];
    for statement in &file.statements {
        if let Some(procedure) = statement.as_procedure() {
            let proc_type = Type::Procedure {
                parameters: procedure
                    .parameters
                    .iter()
                    .map(|parameter| Ok(eval_type(&parameter.typ)?))
                    .collect::<Result<_, _>>()?,
                return_type: Box::new(eval_type(&procedure.return_type)?),
            };
            resolve_procedure(procedure, &proc_type, &mut variables, &mut procedures)?;
        } else {
            return Err(ResolvingError::NonProcedureAtGlobalScope {
                location: statement.get_location().clone(),
            });
        }
    }
    Ok(BytecodeProgram {
        procedures,
        main_proc_index: variables
            .get("main")
            .map(|(_, _, index)| index.clone())
            .ok_or_else(|| ResolvingError::NoMainProcedure)?,
    })
}

fn resolve_procedure(
    procedure: &Rc<AstProcedure>,
    proc_type: &Type,
    variables: &mut HashMap<String, (Declaration, Type, usize)>,
    procedures: &mut Vec<BytecodeProcedure>,
) -> Result<(), ResolvingError> {
    if let Some((variable, _, _)) = variables.get(&procedure.name) {
        return Err(ResolvingError::Redeclaration {
            name: procedure.name.clone(),
            old_declaration: variable.get_location().clone(),
            new_declaration: procedure.location.clone(),
        });
    }

    let proc_index = procedures.len();
    variables.insert(
        procedure.name.clone(),
        (
            Declaration::Procedure(procedure.clone()),
            proc_type.clone(),
            proc_index,
        ),
    );
    procedures.push(BytecodeProcedure {
        instructions: vec![],
        max_registers: 0,
    });

    let mut variables = variables
        .iter()
        .filter_map(|(name, (decl, typ, index))| match decl {
            Declaration::Procedure(_) => {
                Some((name.clone(), (decl.clone(), typ.clone(), index.clone())))
            }
            Declaration::Parameter(_) => None,
            Declaration::Let(_) => None,
            Declaration::Var(_) => None,
        })
        .collect::<HashMap<_, _>>();

    let mut instructions = vec![];
    let mut max_registers = procedure.parameters.len();
    let mut next_register = procedure.parameters.len();

    for (i, parameter) in procedure.parameters.iter().enumerate() {
        if let Some((variable, _, _)) = variables.get(&parameter.name) {
            return Err(ResolvingError::Redeclaration {
                name: parameter.name.clone(),
                old_declaration: variable.get_location().clone(),
                new_declaration: parameter.location.clone(),
            });
        }
        variables.insert(
            parameter.name.clone(),
            (
                Declaration::Parameter(parameter.clone()),
                eval_type(&parameter.typ)?,
                i,
            ),
        );
    }

    match &procedure.body {
        ProcedureBody::CompilerGenerated(_) => match &procedure.name as &str {
            "print_int" => {
                expect_types_equal(
                    proc_type,
                    &Type::Procedure {
                        parameters: vec![Type::Int],
                        return_type: Box::new(Type::Void),
                    },
                    &procedure.location,
                )?;
                instructions.push(BytecodeInstruction::PrintInt { reg: 0 });
                let ret_value = allocate_register(&mut max_registers, &mut next_register);
                instructions.push(BytecodeInstruction::Set {
                    dest: ret_value,
                    value: BytecodeValue::Void,
                });
                instructions.push(BytecodeInstruction::Return { reg: ret_value });
            }

            "println" => {
                expect_types_equal(
                    proc_type,
                    &Type::Procedure {
                        parameters: vec![],
                        return_type: Box::new(Type::Void),
                    },
                    &procedure.location,
                )?;
                instructions.push(BytecodeInstruction::PrintLn);
                let ret_value = allocate_register(&mut max_registers, &mut next_register);
                instructions.push(BytecodeInstruction::Set {
                    dest: ret_value,
                    value: BytecodeValue::Void,
                });
                instructions.push(BytecodeInstruction::Return { reg: ret_value });
            }

            _ => {
                return Err(ResolvingError::UnknownCompilerProcedure {
                    location: procedure.location.clone(),
                    name: procedure.name.clone(),
                })
            }
        },
        ProcedureBody::Scope(scope) => {
            let return_type = *proc_type.as_procedure().unwrap().1.clone();
            resolve_ast(
                &scope,
                &mut instructions,
                &mut max_registers,
                &mut next_register,
                &mut variables,
                procedures,
                &Some(return_type.clone()),
            )?;
            if return_type == Type::Void {
                let ret_value = allocate_register(&mut max_registers, &mut next_register);
                instructions.push(BytecodeInstruction::Set {
                    dest: ret_value,
                    value: BytecodeValue::Void,
                });
                instructions.push(BytecodeInstruction::Return { reg: ret_value });
            } else {
                if !returns_in_all_paths(scope) {
                    return Err(ResolvingError::NoReturn {
                        location: procedure.location.clone(),
                    });
                }
            }
        }
    }

    procedures[proc_index] = BytecodeProcedure {
        instructions,
        max_registers,
    };
    Ok(())
}

#[derive(Clone, Debug, PartialEq, EnumAsInner)]
enum Declaration {
    Procedure(Rc<AstProcedure>),
    Parameter(Rc<Parameter>),
    Let(Rc<AstLet>),
    Var(Rc<AstVar>),
}

impl Declaration {
    fn get_location(&self) -> &SourceSpan {
        match self {
            Declaration::Procedure(procedure) => &procedure.location,
            Declaration::Parameter(parameter) => &parameter.location,
            Declaration::Let(lett) => &lett.location,
            Declaration::Var(var) => &var.location,
        }
    }
}

fn resolve_ast(
    ast: &Ast,
    instructions: &mut Vec<BytecodeInstruction>,
    max_registers: &mut usize,
    next_register: &mut usize,
    variables: &mut HashMap<String, (Declaration, Type, usize)>,
    procedures: &mut Vec<BytecodeProcedure>,
    proc_return_type: &Option<Type>,
) -> Result<Option<(usize, Type)>, ResolvingError> {
    Ok(match ast {
        Ast::File(_) => unreachable!(),

        Ast::Procedure(procedure) => {
            let proc_type = Type::Procedure {
                parameters: procedure
                    .parameters
                    .iter()
                    .map(|parameter| Ok(eval_type(&parameter.typ)?))
                    .collect::<Result<_, _>>()?,
                return_type: Box::new(eval_type(&procedure.return_type)?),
            };
            resolve_procedure(procedure, &proc_type, variables, procedures)?;
            None
        }

        Ast::Return(returnn) => {
            let (value, value_typ) = if let Some(value) = &returnn.value {
                resolve_ast(
                    value,
                    instructions,
                    max_registers,
                    next_register,
                    variables,
                    procedures,
                    proc_return_type,
                )?
                .unwrap()
            } else {
                let void = allocate_register(max_registers, next_register);
                instructions.push(BytecodeInstruction::Set {
                    dest: void,
                    value: BytecodeValue::Void,
                });
                (void, Type::Void)
            };
            expect_types_equal(
                &value_typ,
                proc_return_type
                    .as_ref()
                    .expect("we should only be here we are inside of a procedure"),
                &returnn.location,
            )?;
            instructions.push(BytecodeInstruction::Return { reg: value });
            None
        }

        Ast::Scope(scope) => {
            let mut next_register_copy = *next_register;
            let mut variables_copy = variables.clone();
            for statement in &scope.statements {
                resolve_ast(
                    statement,
                    instructions,
                    max_registers,
                    &mut next_register_copy,
                    &mut variables_copy,
                    procedures,
                    proc_return_type,
                )?;
            }
            None
        }

        Ast::Let(lett) => {
            let reg = allocate_register(max_registers, next_register);

            let decl_typ = if let Some(typ) = &lett.typ {
                Some(eval_type(typ)?)
            } else {
                None
            };
            let (value_reg, value_typ) = resolve_ast(
                &lett.value,
                instructions,
                max_registers,
                next_register,
                variables,
                procedures,
                proc_return_type,
            )?
            .unwrap();
            instructions.push(BytecodeInstruction::Move {
                dest: reg,
                src: value_reg,
            });

            let typ = if let Some(typ) = decl_typ {
                expect_types_equal(&value_typ, &typ, lett.value.get_location())?;
                typ
            } else {
                value_typ
            };

            if let Some((variable, _, _)) = variables.get(&lett.name) {
                return Err(ResolvingError::Redeclaration {
                    name: lett.name.clone(),
                    old_declaration: variable.get_location().clone(),
                    new_declaration: lett.location.clone(),
                });
            }

            variables.insert(
                lett.name.clone(),
                (Declaration::Let(lett.clone()), typ.clone(), reg),
            );

            Some((reg, typ))
        }

        Ast::Var(var) => {
            let reg = allocate_register(max_registers, next_register);

            let decl_typ = if let Some(typ) = &var.typ {
                Some(eval_type(typ)?)
            } else {
                None
            };
            let (value_reg, value_typ) = resolve_ast(
                &var.value,
                instructions,
                max_registers,
                next_register,
                variables,
                procedures,
                proc_return_type,
            )?
            .unwrap();
            instructions.push(BytecodeInstruction::Move {
                dest: reg,
                src: value_reg,
            });

            let typ = if let Some(typ) = decl_typ {
                expect_types_equal(&value_typ, &typ, var.value.get_location())?;
                typ
            } else {
                value_typ
            };

            if let Some((variable, _, _)) = variables.get(&var.name) {
                return Err(ResolvingError::Redeclaration {
                    name: var.name.clone(),
                    old_declaration: variable.get_location().clone(),
                    new_declaration: var.location.clone(),
                });
            }

            variables.insert(
                var.name.clone(),
                (Declaration::Var(var.clone()), typ.clone(), reg),
            );

            Some((reg, typ))
        }

        Ast::LeftAssign(left_assign) => {
            if !is_assignable(&left_assign.operand, variables) {
                return Err(ResolvingError::NotAssignable {
                    location: left_assign.operand.get_location().clone(),
                });
            }
            let (operand, operand_typ) = resolve_ast(
                &left_assign.operand,
                instructions,
                max_registers,
                next_register,
                variables,
                procedures,
                proc_return_type,
            )?
            .unwrap();
            let (value, value_typ) = resolve_ast(
                &left_assign.value,
                instructions,
                max_registers,
                next_register,
                variables,
                procedures,
                proc_return_type,
            )?
            .unwrap();
            expect_types_equal(&value_typ, &operand_typ, &left_assign.location)?;
            instructions.push(BytecodeInstruction::Move {
                dest: operand,
                src: value,
            });
            None
        }

        Ast::RightAssign(right_assign) => {
            if !is_assignable(&right_assign.operand, variables) {
                return Err(ResolvingError::NotAssignable {
                    location: right_assign.operand.get_location().clone(),
                });
            }
            let (value, value_typ) = resolve_ast(
                &right_assign.value,
                instructions,
                max_registers,
                next_register,
                variables,
                procedures,
                proc_return_type,
            )?
            .unwrap();
            let (operand, operand_typ) = resolve_ast(
                &right_assign.operand,
                instructions,
                max_registers,
                next_register,
                variables,
                procedures,
                proc_return_type,
            )?
            .unwrap();
            expect_types_equal(&value_typ, &operand_typ, &right_assign.location)?;
            instructions.push(BytecodeInstruction::Move {
                dest: operand,
                src: value,
            });
            None
        }

        Ast::If(iff) => {
            let (condition, condition_typ) = resolve_ast(
                &iff.condition,
                instructions,
                max_registers,
                next_register,
                variables,
                procedures,
                proc_return_type,
            )?
            .unwrap();
            expect_types_equal(&condition_typ, &Type::Bool, &iff.location)?;
            instructions.push(BytecodeInstruction::LogicalNot {
                dest: condition,
                reg: condition,
            });
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
                variables,
                procedures,
                proc_return_type,
            )?;
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
                    variables,
                    procedures,
                    proc_return_type,
                )?;
            }
            *instructions[jump_past_else_location].as_jump_mut().unwrap() = instructions.len();
            None
        }

        Ast::While(whilee) => {
            let jump_location = instructions.len();
            let (condition, condition_typ) = resolve_ast(
                &whilee.condition,
                instructions,
                max_registers,
                next_register,
                variables,
                procedures,
                proc_return_type,
            )?
            .unwrap();
            expect_types_equal(&condition_typ, &Type::Bool, &whilee.location)?;
            instructions.push(BytecodeInstruction::LogicalNot {
                dest: condition,
                reg: condition,
            });
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
                variables,
                procedures,
                proc_return_type,
            )?;
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
                variables,
                procedures,
                proc_return_type,
            )?
            .unwrap();
            let mut arguments = vec![];
            let (param_types, return_type) = if let Some(proc_type) = operand_typ.as_procedure() {
                proc_type
            } else {
                return Err(ResolvingError::ExpectedProcedure {
                    location: call.operand.get_location().clone(),
                    typ: operand_typ,
                });
            };
            for (i, argument) in call.arguments.iter().enumerate() {
                let (argument_reg, argument_typ) = resolve_ast(
                    argument,
                    instructions,
                    max_registers,
                    next_register,
                    variables,
                    procedures,
                    proc_return_type,
                )?
                .unwrap();
                expect_types_equal(&argument_typ, &param_types[i], argument.get_location())?;
                arguments.push(argument_reg);
            }
            let ret = allocate_register(max_registers, next_register);
            instructions.push(BytecodeInstruction::Call {
                proc: operand,
                dest: ret,
                args: arguments,
            });
            Some((ret, *return_type.clone()))
        }

        Ast::Unary(unary) => {
            let (operand, operand_typ) = resolve_ast(
                &unary.operand,
                instructions,
                max_registers,
                next_register,
                variables,
                procedures,
                proc_return_type,
            )?
            .unwrap();
            let reg = allocate_register(max_registers, next_register);
            match unary.operator {
                UnaryOperator::Identity => {
                    expect_types_equal(&operand_typ, &Type::Int, unary.operand.get_location())?;
                    Some((reg, Type::Int))
                }
                UnaryOperator::Negation => {
                    expect_types_equal(&operand_typ, &Type::Int, unary.operand.get_location())?;
                    instructions.push(BytecodeInstruction::Negate {
                        dest: reg,
                        reg: operand,
                    });
                    Some((reg, Type::Int))
                }
            }
        }

        Ast::Binary(binary) => {
            let (left, left_typ) = resolve_ast(
                &binary.left,
                instructions,
                max_registers,
                next_register,
                variables,
                procedures,
                proc_return_type,
            )?
            .unwrap();
            let (right, right_typ) = resolve_ast(
                &binary.right,
                instructions,
                max_registers,
                next_register,
                variables,
                procedures,
                proc_return_type,
            )?
            .unwrap();
            let reg = allocate_register(max_registers, next_register);
            match binary.operator {
                BinaryOperator::Add => {
                    expect_types_equal(&left_typ, &Type::Int, binary.left.get_location())?;
                    expect_types_equal(&right_typ, &Type::Int, binary.right.get_location())?;
                    instructions.push(BytecodeInstruction::Add {
                        dest: reg,
                        a: left,
                        b: right,
                    });
                    Some((reg, Type::Int))
                }

                BinaryOperator::Subtract => {
                    expect_types_equal(&left_typ, &Type::Int, binary.left.get_location())?;
                    expect_types_equal(&right_typ, &Type::Int, binary.right.get_location())?;
                    instructions.push(BytecodeInstruction::Subtract {
                        dest: reg,
                        a: left,
                        b: right,
                    });
                    Some((reg, Type::Int))
                }

                BinaryOperator::Multiply => {
                    expect_types_equal(&left_typ, &Type::Int, binary.left.get_location())?;
                    expect_types_equal(&right_typ, &Type::Int, binary.right.get_location())?;
                    instructions.push(BytecodeInstruction::Multiply {
                        dest: reg,
                        a: left,
                        b: right,
                    });
                    Some((reg, Type::Int))
                }

                BinaryOperator::Divide => {
                    expect_types_equal(&left_typ, &Type::Int, binary.left.get_location())?;
                    expect_types_equal(&right_typ, &Type::Int, binary.right.get_location())?;
                    instructions.push(BytecodeInstruction::Divide {
                        dest: reg,
                        a: left,
                        b: right,
                    });
                    Some((reg, Type::Int))
                }

                BinaryOperator::LessThan => {
                    expect_types_equal(&left_typ, &Type::Int, binary.left.get_location())?;
                    expect_types_equal(&right_typ, &Type::Int, binary.right.get_location())?;
                    instructions.push(BytecodeInstruction::LessThan {
                        dest: reg,
                        a: left,
                        b: right,
                    });
                    Some((reg, Type::Bool))
                }

                BinaryOperator::GreaterThan => {
                    expect_types_equal(&left_typ, &Type::Int, binary.left.get_location())?;
                    expect_types_equal(&right_typ, &Type::Int, binary.right.get_location())?;
                    instructions.push(BytecodeInstruction::GreaterThan {
                        dest: reg,
                        a: left,
                        b: right,
                    });
                    Some((reg, Type::Bool))
                }

                BinaryOperator::LessThanEqual => {
                    expect_types_equal(&left_typ, &Type::Int, binary.left.get_location())?;
                    expect_types_equal(&right_typ, &Type::Int, binary.right.get_location())?;
                    instructions.push(BytecodeInstruction::GreaterThan {
                        dest: reg,
                        a: left,
                        b: right,
                    });
                    instructions.push(BytecodeInstruction::LogicalNot { dest: reg, reg });
                    Some((reg, Type::Bool))
                }

                BinaryOperator::GreaterThanEqual => {
                    expect_types_equal(&left_typ, &Type::Int, binary.left.get_location())?;
                    expect_types_equal(&right_typ, &Type::Int, binary.right.get_location())?;
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

        Ast::Name(name) => Some(
            if let Some((decl, typ, index)) = variables.get(&name.name) {
                (
                    match decl {
                        Declaration::Procedure(_) => {
                            let reg = allocate_register(max_registers, next_register);
                            instructions.push(BytecodeInstruction::Set {
                                dest: reg,
                                value: BytecodeValue::Procedure(*index),
                            });
                            reg
                        }
                        _ => *index,
                    },
                    typ.clone(),
                )
            } else {
                return Err(ResolvingError::UndeclaredName {
                    name: name.name.clone(),
                    name_location: name.location.clone(),
                });
            },
        ),

        Ast::Integer(integer) => {
            let reg = allocate_register(max_registers, next_register);
            if integer.integer > i64::MAX as u128 {
                return Err(ResolvingError::IntegerTooBigForType {
                    value: integer.integer,
                    typ: Type::Int,
                    integer_location: integer.location.clone(),
                });
            }
            instructions.push(BytecodeInstruction::Set {
                dest: reg,
                value: BytecodeValue::Int(integer.integer as i64),
            });
            Some((reg, Type::Int))
        }
    })
}

fn eval_type(ast: &Ast) -> Result<Type, ResolvingError> {
    Ok(match ast {
        Ast::Name(name) => match &name.name as &str {
            "void" => Type::Void,
            "int" => Type::Int,
            "bool" => Type::Bool,
            _ => {
                return Err(ResolvingError::UndeclaredType {
                    name: name.name.clone(),
                    name_location: name.location.clone(),
                })
            }
        },
        _ => {
            return Err(ResolvingError::ExpectedType {
                location: ast.get_location().clone(),
            })
        }
    })
}

fn allocate_register(max_registers: &mut usize, next_register: &mut usize) -> usize {
    let register = *next_register;
    *next_register += 1;
    if next_register > max_registers {
        *max_registers = *next_register;
    }
    register
}

fn expect_types_equal(
    typ: &Type,
    expected_typ: &Type,
    location: &SourceSpan,
) -> Result<(), ResolvingError> {
    if typ != expected_typ {
        Err(ResolvingError::ExpectedTypeButGotType {
            location: location.clone(),
            expected_typ: expected_typ.clone(),
            typ: typ.clone(),
        })
    } else {
        Ok(())
    }
}

fn is_assignable(ast: &Ast, variables: &HashMap<String, (Declaration, Type, usize)>) -> bool {
    match ast {
        Ast::File(_) => false,
        Ast::Procedure(_) => false,
        Ast::Return(_) => false,
        Ast::Scope(_) => false,
        Ast::Let(_) => false,
        Ast::Var(_) => false,
        Ast::LeftAssign(_) => false,
        Ast::RightAssign(_) => false,
        Ast::If(_) => false,
        Ast::While(_) => false,
        Ast::Call(_) => false,
        Ast::Unary(_) => false,
        Ast::Binary(_) => false,
        Ast::Name(name) => {
            if let Some((decl, _, _)) = variables.get(&name.name) {
                match decl {
                    Declaration::Procedure(_) => false,
                    Declaration::Parameter(_) => false,
                    Declaration::Let(_) => false,
                    Declaration::Var(_) => true,
                }
            } else {
                false
            }
        }
        Ast::Integer(_) => false,
    }
}

fn returns_in_all_paths(ast: &Ast) -> bool {
    match ast {
        Ast::File(_) => unreachable!(),
        Ast::Procedure(_) => false,
        Ast::Return(_) => true,
        Ast::Scope(scope) => scope
            .statements
            .iter()
            .any(|statement| returns_in_all_paths(statement)),
        Ast::Let(_) => false,
        Ast::Var(_) => false,
        Ast::LeftAssign(_) => false,
        Ast::RightAssign(_) => false,
        Ast::If(iff) => {
            iff.else_statement
                .as_ref()
                .map(|statement| returns_in_all_paths(statement))
                .unwrap_or(false)
                && returns_in_all_paths(&iff.then_statement)
        }
        Ast::While(whilee) => returns_in_all_paths(&whilee.body),
        Ast::Call(_) => false,
        Ast::Unary(_) => false,
        Ast::Binary(_) => false,
        Ast::Name(_) => false,
        Ast::Integer(_) => false,
    }
}
