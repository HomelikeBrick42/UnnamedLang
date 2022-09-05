use std::{collections::HashMap, rc::Rc};

use derive_more::{Display, IsVariant};
use enum_as_inner::EnumAsInner;

use crate::{
    get_or_add_type_bool, get_or_add_type_integer, get_or_add_type_pointer,
    get_or_add_type_procedure, get_or_add_type_type, get_or_add_type_void, Ast, AstAssignDirection,
    AstBuiltin, AstBuiltinKind, AstLet, AstParameter, AstProcedure, AstProcedureBody, AstVar,
    BinaryOperator, SourceSpan, Type, UnaryOperator,
};

#[derive(Clone, Debug, Display, PartialEq, IsVariant, EnumAsInner)]
pub enum ResolvingError {
    #[display(fmt = "{new}: Redeclaration of '{name}', the original declaration was here: {old}")]
    Redeclaration {
        name: String,
        new: SourceSpan,
        old: SourceSpan,
    },
    #[display(fmt = "{location}: '{name}' is undeclared")]
    UndeclaredName { name: String, location: SourceSpan },
    #[display(fmt = "{location}: Expected type '{expected}', but got type '{got}'")]
    ExpectedType {
        expected: Rc<Type>,
        got: Rc<Type>,
        location: SourceSpan,
    },
    #[display(
        fmt = "{}: '{}' does not return in all control paths",
        "procedure.location",
        "procedure.name"
    )]
    ProcedureNoReturn { procedure: Rc<AstProcedure> },
    #[display(fmt = "{location}: Operand is not assignable")]
    NotAssignable { location: SourceSpan },
}

#[derive(Clone, Debug, PartialEq, IsVariant, EnumAsInner)]
pub enum Declaration {
    Procedure(Rc<AstProcedure>),
    Parameter(Rc<AstParameter>),
    Let(Rc<AstLet>),
    Var(Rc<AstVar>),
    Builtin(Rc<AstBuiltin>),
}

impl Declaration {
    pub fn is_visible_through_procedures(&self) -> bool {
        matches!(self, Declaration::Procedure(_) | Declaration::Builtin(_))
    }

    pub fn to_ast(&self) -> Ast {
        match self {
            Declaration::Procedure(procedure) => Ast::Procedure(procedure.clone()),
            Declaration::Parameter(parameter) => Ast::Parameter(parameter.clone()),
            Declaration::Let(declaration) => Ast::LetDeclaration(declaration.clone()),
            Declaration::Var(declaration) => Ast::VarDeclaration(declaration.clone()),
            Declaration::Builtin(builtin) => Ast::Builtin(builtin.clone()),
        }
    }
}

fn is_assignable(ast: &Ast) -> bool {
    match ast {
        Ast::File(_) => false,
        Ast::Procedure(_) => false,
        Ast::ProcedureType(_) => false,
        Ast::Parameter(parameter) => parameter.mutable,
        Ast::Scope(_) => false,
        Ast::LetDeclaration(_) => false,
        Ast::VarDeclaration(_) => true,
        Ast::Name(name) => is_assignable(name.resolved_declaration.borrow().as_ref().unwrap()),
        Ast::Integer(_) => false,
        Ast::Call(_) => false,
        Ast::Return(_) => false,
        Ast::Unary(unary) => match &unary.operator {
            UnaryOperator::Identity => false,
            UnaryOperator::Negation => false,
            UnaryOperator::LogicalNot => false,
            UnaryOperator::PointerType => false,
            UnaryOperator::AddressOf => false,
            UnaryOperator::Dereference => true,
        },
        Ast::Binary(_) => false,
        Ast::If(_) => false,
        Ast::While(_) => false,
        Ast::Cast(_) => false,
        Ast::Assign(_) => false,
        Ast::Builtin(_) => false,
    }
}

fn is_addressable(ast: &Ast) -> bool {
    match ast {
        Ast::File(_) => false,
        Ast::Procedure(_) => false,
        Ast::ProcedureType(_) => true,
        Ast::Parameter(parameter) => parameter.mutable,
        Ast::Scope(_) => true,
        Ast::LetDeclaration(_) => false,
        Ast::VarDeclaration(_) => true,
        Ast::Name(name) => is_addressable(name.resolved_declaration.borrow().as_ref().unwrap()),
        Ast::Integer(_) => true,
        Ast::Call(_) => true,
        Ast::Return(_) => false,
        Ast::Unary(_) => true,
        Ast::Binary(_) => true,
        Ast::If(_) => true,
        Ast::While(_) => true,
        Ast::Cast(_) => true,
        Ast::Assign(_) => true,
        Ast::Builtin(_) => false,
    }
}

pub fn resolve_names(
    ast: &Ast,
    names: &mut HashMap<String, Declaration>,
) -> Result<(), ResolvingError> {
    fn scope_like(
        expressions: &[Ast],
        names: &mut HashMap<String, Declaration>,
    ) -> Result<(), ResolvingError> {
        for expression in expressions {
            match expression {
                Ast::Procedure(procedure) => {
                    if let Some(old) = names.insert(
                        procedure.name.clone(),
                        Declaration::Procedure(procedure.clone()),
                    ) {
                        return Err(ResolvingError::Redeclaration {
                            name: procedure.name.clone(),
                            new: procedure.location.clone(),
                            old: old.to_ast().get_location(),
                        });
                    }
                }
                _ => (),
            }
        }
        for expression in expressions {
            resolve_names(expression, &mut names.clone())?;
            match expression {
                Ast::File(_) => (),
                Ast::Procedure(_) => (), // procedures are already declared
                Ast::ProcedureType(_) => (),
                Ast::Parameter(_) => (),
                Ast::Scope(_) => (),
                Ast::LetDeclaration(declaration) => {
                    if let Some(old) = names.insert(
                        declaration.name.clone(),
                        Declaration::Let(declaration.clone()),
                    ) {
                        return Err(ResolvingError::Redeclaration {
                            name: declaration.name.clone(),
                            new: declaration.location.clone(),
                            old: old.to_ast().get_location(),
                        });
                    }
                }
                Ast::VarDeclaration(declaration) => {
                    if let Some(old) = names.insert(
                        declaration.name.clone(),
                        Declaration::Var(declaration.clone()),
                    ) {
                        return Err(ResolvingError::Redeclaration {
                            name: declaration.name.clone(),
                            new: declaration.location.clone(),
                            old: old.to_ast().get_location(),
                        });
                    }
                }
                Ast::Name(_) => (),
                Ast::Integer(_) => (),
                Ast::Call(_) => (),
                Ast::Return(_) => (),
                Ast::Unary(_) => (),
                Ast::Binary(_) => (),
                Ast::If(_) => (),
                Ast::While(_) => (),
                Ast::Cast(_) => (),
                Ast::Assign(_) => (),
                Ast::Builtin(_) => (),
            }
        }
        Ok(())
    }

    Ok(match ast {
        Ast::File(file) => {
            scope_like(&file.expressions, &mut names.clone())?;
        }
        Ast::Procedure(procedure) => {
            if !names.contains_key(&procedure.name) {
                if let Some(old) = names.insert(
                    procedure.name.clone(),
                    Declaration::Procedure(procedure.clone()),
                ) {
                    return Err(ResolvingError::Redeclaration {
                        name: procedure.name.clone(),
                        new: procedure.location.clone(),
                        old: old.to_ast().get_location(),
                    });
                }
            }
            let mut names = names
                .iter()
                .filter(|(_, decl)| decl.is_visible_through_procedures())
                .map(|(name, decl)| (name.clone(), decl.clone()))
                .collect::<HashMap<String, Declaration>>();
            for parameter in &procedure.parameters {
                resolve_names(&Ast::Parameter(parameter.clone()), &mut names)?;
            }
            resolve_names(&procedure.return_type, &mut names)?;
            match &procedure.body {
                AstProcedureBody::ExternName(_) => (),
                AstProcedureBody::Scope(scope) => {
                    resolve_names(&Ast::Scope(scope.clone()), &mut names)?;
                }
            }
        }
        Ast::ProcedureType(procedure_type) => {
            for parameter in &procedure_type.parameter_types {
                resolve_names(parameter, names)?;
            }
            resolve_names(&procedure_type.return_type, names)?;
        }
        Ast::Parameter(parameter) => {
            resolve_names(&parameter.typ, names)?;
            if let Some(old) = names.insert(
                parameter.name.clone(),
                Declaration::Parameter(parameter.clone()),
            ) {
                return Err(ResolvingError::Redeclaration {
                    name: parameter.name.clone(),
                    new: parameter.location.clone(),
                    old: old.to_ast().get_location(),
                });
            }
        }
        Ast::Scope(scope) => {
            scope_like(&scope.expressions, &mut names.clone())?;
        }
        Ast::LetDeclaration(declaration) => {
            if let Some(typ) = &declaration.typ {
                resolve_names(typ, names)?;
            }
            resolve_names(&declaration.value, names)?;
            if let Some(old) = names.insert(
                declaration.name.clone(),
                Declaration::Let(declaration.clone()),
            ) {
                return Err(ResolvingError::Redeclaration {
                    name: declaration.name.clone(),
                    new: declaration.location.clone(),
                    old: old.to_ast().get_location(),
                });
            }
        }
        Ast::VarDeclaration(declaration) => {
            if let Some(typ) = &declaration.typ {
                resolve_names(typ, names)?;
            }
            resolve_names(&declaration.value, names)?;
            if let Some(old) = names.insert(
                declaration.name.clone(),
                Declaration::Var(declaration.clone()),
            ) {
                return Err(ResolvingError::Redeclaration {
                    name: declaration.name.clone(),
                    new: declaration.location.clone(),
                    old: old.to_ast().get_location(),
                });
            }
        }
        Ast::Name(name) => {
            if name.resolved_declaration.borrow().is_none() {
                let decl = if let Some(decl) = names.get(&name.name) {
                    decl
                } else {
                    return Err(ResolvingError::UndeclaredName {
                        name: name.name.clone(),
                        location: name.location.clone(),
                    });
                };
                *name.resolved_declaration.borrow_mut() = Some(decl.to_ast());
            }
        }
        Ast::Integer(_) => (),
        Ast::Call(call) => {
            resolve_names(&call.operand, names)?;
            for argument in &call.arguments {
                resolve_names(argument, names)?;
            }
        }
        Ast::Return(returnn) => {
            if let Some(value) = &returnn.value {
                resolve_names(value, names)?;
            }
        }
        Ast::Unary(unary) => {
            resolve_names(&unary.operand, names)?;
        }
        Ast::Binary(binary) => {
            resolve_names(&binary.left, names)?;
            resolve_names(&binary.right, names)?;
        }
        Ast::If(iff) => {
            resolve_names(&iff.condition, names)?;
            resolve_names(&iff.then_expression, names)?;
            if let Some(else_expression) = &iff.else_expression {
                resolve_names(else_expression, names)?;
            }
        }
        Ast::While(whilee) => {
            resolve_names(&whilee.condition, names)?;
            resolve_names(&whilee.then_expression, names)?;
        }
        Ast::Cast(cast) => {
            resolve_names(&cast.typ, names)?;
            resolve_names(&cast.operand, names)?;
        }
        Ast::Assign(assign) => match &assign.direction {
            AstAssignDirection::Left => {
                resolve_names(&assign.operand, names)?;
                resolve_names(&assign.value, names)?;
            }
            AstAssignDirection::Right => {
                resolve_names(&assign.value, names)?;
                resolve_names(&assign.operand, names)?;
            }
        },
        Ast::Builtin(builtin) => match &builtin.kind {
            AstBuiltinKind::Type => (),
            AstBuiltinKind::Void => (),
            AstBuiltinKind::Bool => (),
            AstBuiltinKind::IntegerType { size: _, signed: _ } => (),
        },
    })
}

fn expect_type(
    typ: &Rc<Type>,
    expected: &Rc<Type>,
    location: SourceSpan,
) -> Result<(), ResolvingError> {
    if typ == expected {
        Ok(())
    } else {
        Err(ResolvingError::ExpectedType {
            expected: expected.clone(),
            got: typ.clone(),
            location,
        })
    }
}

fn eval_type(ast: &Ast, type_cache: &mut Vec<Rc<Type>>) -> Result<Rc<Type>, ResolvingError> {
    Ok(match ast {
        Ast::File(_) => todo!(),
        Ast::Procedure(_) => todo!(),
        Ast::ProcedureType(procedure_type) => {
            let parameter_types = procedure_type
                .parameter_types
                .iter()
                .map(|typ| eval_type(typ, type_cache))
                .collect::<Result<_, _>>()?;
            let return_type = eval_type(&procedure_type.return_type, type_cache)?;
            get_or_add_type_procedure(
                type_cache,
                parameter_types,
                return_type,
                procedure_type.calling_convention.clone(),
            )
        }
        Ast::Parameter(_) => todo!(),
        Ast::Scope(_) => todo!(),
        Ast::LetDeclaration(_) => todo!(),
        Ast::VarDeclaration(_) => todo!(),
        Ast::Name(name) => eval_type(
            name.resolved_declaration.borrow().as_ref().unwrap(),
            type_cache,
        )?,
        Ast::Integer(_) => todo!(),
        Ast::Call(_) => todo!(),
        Ast::Return(_) => todo!(),
        Ast::Unary(unary) => match &unary.operator {
            UnaryOperator::Identity => todo!(),
            UnaryOperator::Negation => todo!(),
            UnaryOperator::LogicalNot => todo!(),
            UnaryOperator::PointerType => {
                let pointed_to = eval_type(&unary.operand, type_cache)?;
                get_or_add_type_pointer(type_cache, pointed_to)
            }
            .into(),
            UnaryOperator::AddressOf => todo!(),
            UnaryOperator::Dereference => todo!(),
        },
        Ast::Binary(_) => todo!(),
        Ast::If(_) => todo!(),
        Ast::While(_) => todo!(),
        Ast::Cast(_) => todo!(),
        Ast::Assign(_) => todo!(),
        Ast::Builtin(builtin) => match &builtin.kind {
            AstBuiltinKind::Type => get_or_add_type_type(type_cache),
            AstBuiltinKind::Void => get_or_add_type_void(type_cache),
            AstBuiltinKind::Bool => get_or_add_type_bool(type_cache),
            &AstBuiltinKind::IntegerType { size, signed } => {
                get_or_add_type_integer(type_cache, size, signed)
            }
        },
    })
}

pub fn resolve(
    ast: &Ast,
    suggested_type: Option<Rc<Type>>,
    defered_asts: &mut Vec<(Option<Rc<AstProcedure>>, Ast)>,
    parent_procedure: &Option<Rc<AstProcedure>>,
    type_cache: &mut Vec<Rc<Type>>,
) -> Result<Rc<Type>, ResolvingError> {
    Ok(if let Some(typ) = ast.get_type() {
        typ
    } else {
        if ast.get_resolving() {
            todo!("cyclic dependency found")
        }
        ast.set_resolving(true);
        match ast {
            Ast::File(file) => {
                *file.resolved_type.borrow_mut() = Some(get_or_add_type_void(type_cache));
                for expression in &file.expressions {
                    resolve(expression, None, defered_asts, &None, type_cache)?;
                }
                while let Some((parent_procedure, ast)) = defered_asts.pop() {
                    resolve(&ast, None, defered_asts, &parent_procedure, type_cache)?;
                }
            }
            Ast::Procedure(procedure) => {
                let suggested_proc_type = suggested_type
                    .as_ref()
                    .map(|typ| typ.as_procedure())
                    .flatten();
                let mut parameter_types = vec![];
                for (i, parameter) in procedure.parameters.iter().enumerate() {
                    let suggested_parameter_type =
                        suggested_proc_type.map(|(parameters, _, _)| parameters[i].clone());
                    parameter_types.push(resolve(
                        &Ast::Parameter(parameter.clone()),
                        suggested_parameter_type,
                        defered_asts,
                        &None,
                        type_cache,
                    )?);
                }
                let return_type_type = resolve(
                    &procedure.return_type,
                    Some(get_or_add_type_type(type_cache)),
                    defered_asts,
                    &None,
                    type_cache,
                )?;
                expect_type(
                    &return_type_type,
                    &get_or_add_type_type(type_cache),
                    procedure.return_type.get_location(),
                )?;
                let return_type = eval_type(&procedure.return_type, type_cache)?;
                *procedure.resolved_type.borrow_mut() = Some(get_or_add_type_procedure(
                    type_cache,
                    parameter_types,
                    return_type.clone(),
                    procedure.calling_convention.clone(),
                ));
                match &procedure.body {
                    AstProcedureBody::ExternName(_) => (),
                    AstProcedureBody::Scope(scope) => {
                        fn does_return(ast: &Ast) -> bool {
                            match ast {
                                Ast::File(file) => file.expressions.iter().any(does_return),
                                Ast::Procedure(_) => false,
                                Ast::ProcedureType(_) => false,
                                Ast::Parameter(_) => false,
                                Ast::Scope(scope) => scope.expressions.iter().any(does_return),
                                Ast::LetDeclaration(declaration) => does_return(&declaration.value),
                                Ast::VarDeclaration(declaration) => does_return(&declaration.value),
                                Ast::Name(_) => false,
                                Ast::Integer(_) => false,
                                Ast::Call(call) => {
                                    does_return(&call.operand)
                                        || call.arguments.iter().any(does_return)
                                }
                                Ast::Return(_) => true,
                                Ast::Unary(unary) => does_return(&unary.operand),
                                Ast::Binary(binary) => {
                                    does_return(&binary.left) || does_return(&binary.right)
                                }
                                Ast::If(iff) => {
                                    does_return(&iff.condition)
                                        || iff
                                            .else_expression
                                            .as_ref()
                                            .map(|elsee| {
                                                does_return(&iff.then_expression)
                                                    && does_return(elsee)
                                            })
                                            .unwrap_or(false)
                                }
                                Ast::While(whilee) => {
                                    does_return(&whilee.condition)
                                        || does_return(&whilee.then_expression)
                                }
                                Ast::Assign(assign) => {
                                    does_return(&assign.operand) || does_return(&assign.value)
                                }
                                Ast::Cast(cast) => does_return(&cast.operand),
                                Ast::Builtin(_) => false,
                            }
                        }
                        let scope = Ast::Scope(scope.clone());
                        if !return_type.is_void() && !does_return(&scope) {
                            return Err(ResolvingError::ProcedureNoReturn {
                                procedure: procedure.clone(),
                            });
                        }
                        defered_asts.push((Some(procedure.clone()), scope))
                    }
                }
            }
            Ast::ProcedureType(procedure_type) => {
                *procedure_type.resolved_type.borrow_mut() = Some(get_or_add_type_type(type_cache));
                for parameter in &procedure_type.parameter_types {
                    resolve(&parameter, None, defered_asts, &None, type_cache)?;
                }
                let return_type_type = resolve(
                    &procedure_type.return_type,
                    Some(get_or_add_type_type(type_cache)),
                    defered_asts,
                    &None,
                    type_cache,
                )?;
                expect_type(
                    &return_type_type,
                    &get_or_add_type_type(type_cache),
                    procedure_type.return_type.get_location(),
                )?;
            }
            Ast::Parameter(parameter) => {
                let type_type = resolve(
                    &parameter.typ,
                    Some(get_or_add_type_type(type_cache)),
                    defered_asts,
                    &None,
                    type_cache,
                )?;
                expect_type(
                    &type_type,
                    &get_or_add_type_type(type_cache),
                    parameter.typ.get_location(),
                )?;
                *parameter.resolved_type.borrow_mut() =
                    Some(eval_type(&parameter.typ, type_cache)?);
            }
            Ast::Scope(scope) => {
                *scope.resolved_type.borrow_mut() = Some(get_or_add_type_void(type_cache));
                for expression in &scope.expressions {
                    resolve(expression, None, defered_asts, parent_procedure, type_cache)?;
                }
            }
            Ast::LetDeclaration(declaration) => {
                let suggested_type = if let Some(typ) = &declaration.typ {
                    let type_type = resolve(
                        typ,
                        Some(Type::Type.into()),
                        defered_asts,
                        &None,
                        type_cache,
                    )?;
                    expect_type(
                        &type_type,
                        &get_or_add_type_type(type_cache),
                        typ.get_location(),
                    )?;
                    let resolved_type = eval_type(typ, type_cache)?;
                    *declaration.resolved_type.borrow_mut() = Some(resolved_type.clone());
                    Some(resolved_type)
                } else {
                    suggested_type
                };
                let value_type = resolve(
                    &declaration.value,
                    suggested_type,
                    defered_asts,
                    parent_procedure,
                    type_cache,
                )?;
                if declaration.resolved_type.borrow().is_none() {
                    *declaration.resolved_type.borrow_mut() = Some(value_type);
                } else {
                    expect_type(
                        &value_type,
                        &declaration.resolved_type.borrow().as_ref().unwrap(),
                        declaration.value.get_location(),
                    )?;
                }
            }
            Ast::VarDeclaration(declaration) => {
                let suggested_type = if let Some(typ) = &declaration.typ {
                    let type_type = resolve(
                        typ,
                        Some(get_or_add_type_type(type_cache)),
                        defered_asts,
                        &None,
                        type_cache,
                    )?;
                    expect_type(
                        &type_type,
                        &get_or_add_type_type(type_cache),
                        typ.get_location(),
                    )?;
                    let resolved_type = eval_type(typ, type_cache)?;
                    *declaration.resolved_type.borrow_mut() = Some(resolved_type.clone());
                    Some(resolved_type)
                } else {
                    suggested_type
                };
                let value_type = resolve(
                    &declaration.value,
                    suggested_type,
                    defered_asts,
                    parent_procedure,
                    type_cache,
                )?;
                if declaration.resolved_type.borrow().is_none() {
                    *declaration.resolved_type.borrow_mut() = Some(value_type);
                } else {
                    expect_type(
                        &value_type,
                        &declaration.resolved_type.borrow().as_ref().unwrap(),
                        declaration.value.get_location(),
                    )?;
                }
            }
            Ast::Name(name) => {
                let declaration = name.resolved_declaration.borrow();
                let declaration = declaration
                    .as_ref()
                    .expect("the name should be resolved at this point");
                resolve(
                    &declaration,
                    suggested_type,
                    defered_asts,
                    parent_procedure,
                    type_cache,
                )?;
            }
            Ast::Integer(integer) => {
                // TODO: integer size check
                *integer.resolved_type.borrow_mut() = Some(
                    if suggested_type
                        .as_ref()
                        .map(|typ| typ.as_integer())
                        .flatten()
                        .is_some()
                    {
                        suggested_type.unwrap().clone()
                    } else {
                        get_or_add_type_integer(type_cache, 8, true)
                    },
                );
            }
            Ast::Call(call) => {
                let operand_type = resolve(
                    &call.operand,
                    None,
                    defered_asts,
                    parent_procedure,
                    type_cache,
                )?; // TODO: is there some way we can expect the type here?
                let (parameter_types, return_type, _) =
                    if let Some(procedure_type) = operand_type.as_procedure() {
                        procedure_type
                    } else {
                        todo!("error")
                    };
                *call.resolved_type.borrow_mut() = Some(return_type.clone());
                if call.arguments.len() != parameter_types.len() {
                    todo!("error")
                }
                for (argument, expected_argument_type) in
                    call.arguments.iter().zip(parameter_types.iter())
                {
                    let argument_type = resolve(
                        argument,
                        Some(expected_argument_type.clone()),
                        defered_asts,
                        parent_procedure,
                        type_cache,
                    )?;
                    expect_type(
                        &argument_type,
                        expected_argument_type,
                        argument.get_location(),
                    )?;
                }
            }
            Ast::Return(returnn) => {
                *returnn.resolved_type.borrow_mut() = Some(get_or_add_type_void(type_cache));
                let procedure = if let Some(procedure) = parent_procedure {
                    procedure
                } else {
                    todo!("cannot use return outside of a function")
                };
                let return_type = Ast::Procedure(procedure.clone())
                    .get_type()
                    .unwrap()
                    .as_procedure()
                    .unwrap()
                    .1
                    .clone();
                if let Some(value) = &returnn.value {
                    let value_type = resolve(
                        value,
                        return_type.clone().into(),
                        defered_asts,
                        parent_procedure,
                        type_cache,
                    )?;
                    expect_type(&value_type, &return_type, value.get_location())?;
                } else {
                    expect_type(
                        &get_or_add_type_void(type_cache),
                        &return_type,
                        returnn.location.clone(),
                    )?;
                }
            }
            Ast::Unary(unary) => match &unary.operator {
                UnaryOperator::Identity => {
                    let operand_type = resolve(
                        &unary.operand,
                        suggested_type,
                        defered_asts,
                        parent_procedure,
                        type_cache,
                    )?;
                    if operand_type.as_integer().is_none() {
                        todo!()
                    }
                    *unary.resolved_type.borrow_mut() = Some(operand_type);
                }
                UnaryOperator::Negation => {
                    let operand_type = resolve(
                        &unary.operand,
                        suggested_type,
                        defered_asts,
                        parent_procedure,
                        type_cache,
                    )?;
                    if operand_type.as_integer().is_none() {
                        todo!()
                    }
                    *unary.resolved_type.borrow_mut() = Some(operand_type);
                }
                UnaryOperator::LogicalNot => {
                    let operand_type = resolve(
                        &unary.operand,
                        suggested_type,
                        defered_asts,
                        parent_procedure,
                        type_cache,
                    )?;
                    expect_type(
                        &operand_type,
                        &get_or_add_type_bool(type_cache),
                        unary.operand.get_location(),
                    )?;
                    *unary.resolved_type.borrow_mut() = Some(get_or_add_type_bool(type_cache));
                }
                UnaryOperator::PointerType => {
                    let operand_type = resolve(
                        &unary.operand,
                        suggested_type,
                        defered_asts,
                        parent_procedure,
                        type_cache,
                    )?;
                    expect_type(
                        &operand_type,
                        &get_or_add_type_type(type_cache),
                        unary.operand.get_location(),
                    )?;
                    *unary.resolved_type.borrow_mut() = Some(get_or_add_type_type(type_cache));
                }
                UnaryOperator::AddressOf => {
                    let operand_type = resolve(
                        &unary.operand,
                        suggested_type,
                        defered_asts,
                        parent_procedure,
                        type_cache,
                    )?;
                    if !is_addressable(&unary.operand) {
                        todo!()
                    }
                    *unary.resolved_type.borrow_mut() =
                        Some(get_or_add_type_pointer(type_cache, operand_type));
                }
                UnaryOperator::Dereference => {
                    let operand_type = resolve(
                        &unary.operand,
                        suggested_type.map(|typ| get_or_add_type_pointer(type_cache, typ)),
                        defered_asts,
                        parent_procedure,
                        type_cache,
                    )?;
                    let typ = if let Some(typ) = operand_type.as_pointer() {
                        typ.clone()
                    } else {
                        todo!()
                    };
                    *unary.resolved_type.borrow_mut() = Some(typ);
                }
            },
            Ast::Binary(binary) => {
                let left_type = resolve(
                    &binary.left,
                    suggested_type.clone(),
                    defered_asts,
                    parent_procedure,
                    type_cache,
                )?;
                let right_type = resolve(
                    &binary.right,
                    left_type.clone().into(),
                    defered_asts,
                    parent_procedure,
                    type_cache,
                )?;
                match &binary.operator {
                    BinaryOperator::Add
                    | BinaryOperator::Subtract
                    | BinaryOperator::Multiply
                    | BinaryOperator::Divide
                    | BinaryOperator::Remainder => {
                        if left_type.as_integer().is_none() {
                            todo!()
                        }
                        expect_type(&right_type, &left_type, binary.location.clone())?;
                        *binary.resolved_type.borrow_mut() = Some(left_type);
                    }
                    BinaryOperator::Equal | BinaryOperator::NotEqual => {
                        expect_type(&right_type, &left_type, binary.location.clone())?;
                        *binary.resolved_type.borrow_mut() = Some(get_or_add_type_bool(type_cache));
                    }
                    BinaryOperator::LessThan
                    | BinaryOperator::GreaterThan
                    | BinaryOperator::LessThanEqual
                    | BinaryOperator::GreaterThanEqual => {
                        if left_type.as_integer().is_none() {
                            todo!()
                        }
                        expect_type(&right_type, &left_type, binary.location.clone())?;
                        *binary.resolved_type.borrow_mut() = Some(get_or_add_type_bool(type_cache));
                    }
                }
            }
            Ast::If(iff) => {
                let condition_type = resolve(
                    &iff.condition,
                    Some(get_or_add_type_bool(type_cache)),
                    defered_asts,
                    parent_procedure,
                    type_cache,
                )?;
                expect_type(
                    &condition_type,
                    &get_or_add_type_bool(type_cache),
                    iff.condition.get_location(),
                )?;
                let then_type = resolve(
                    &iff.then_expression,
                    suggested_type.clone(),
                    defered_asts,
                    parent_procedure,
                    type_cache,
                )?;
                if let Some(else_expression) = &iff.else_expression {
                    let else_type = resolve(
                        &else_expression,
                        suggested_type,
                        defered_asts,
                        parent_procedure,
                        type_cache,
                    )?;
                    expect_type(&else_type, &then_type, else_expression.get_location())?;
                } else {
                    expect_type(
                        &then_type,
                        &get_or_add_type_void(type_cache),
                        iff.then_expression.get_location(),
                    )?;
                }
                *iff.resolved_type.borrow_mut() = Some(then_type);
            }
            Ast::While(whilee) => {
                let condition_type = resolve(
                    &whilee.condition,
                    Some(get_or_add_type_bool(type_cache)),
                    defered_asts,
                    parent_procedure,
                    type_cache,
                )?;
                expect_type(
                    &condition_type,
                    &get_or_add_type_bool(type_cache),
                    whilee.condition.get_location(),
                )?;
                let then_type = resolve(
                    &whilee.then_expression,
                    suggested_type.clone(),
                    defered_asts,
                    parent_procedure,
                    type_cache,
                )?;
                expect_type(
                    &then_type,
                    &get_or_add_type_void(type_cache),
                    whilee.then_expression.get_location(),
                )?;
                *whilee.resolved_type.borrow_mut() = Some(then_type);
            }
            Ast::Cast(cast) => {
                let type_type = resolve(
                    &cast.typ,
                    Some(get_or_add_type_type(type_cache)),
                    defered_asts,
                    parent_procedure,
                    type_cache,
                )?;
                expect_type(
                    &type_type,
                    &get_or_add_type_type(type_cache),
                    cast.typ.get_location(),
                )?;
                let typ = eval_type(&cast.typ, type_cache)?;
                let operand_type = resolve(
                    &cast.operand,
                    typ.clone().into(),
                    defered_asts,
                    parent_procedure,
                    type_cache,
                )?;
                if operand_type != typ
                    && !(operand_type.as_integer().is_some() && typ.as_integer().is_some())
                {
                    todo!()
                }
                *cast.resolved_type.borrow_mut() = Some(typ);
            }
            Ast::Assign(assign) => {
                let (operand_type, value_type) = match &assign.direction {
                    AstAssignDirection::Left => {
                        let operand_type = resolve(
                            &assign.operand,
                            suggested_type,
                            defered_asts,
                            parent_procedure,
                            type_cache,
                        )?;
                        let value_type = resolve(
                            &assign.value,
                            operand_type.clone().into(),
                            defered_asts,
                            parent_procedure,
                            type_cache,
                        )?;
                        (operand_type, value_type)
                    }
                    AstAssignDirection::Right => {
                        let value_type = resolve(
                            &assign.value,
                            suggested_type,
                            defered_asts,
                            parent_procedure,
                            type_cache,
                        )?;
                        let operand_type = resolve(
                            &assign.operand,
                            value_type.clone().into(),
                            defered_asts,
                            parent_procedure,
                            type_cache,
                        )?;
                        (operand_type, value_type)
                    }
                };
                expect_type(&value_type, &operand_type, assign.value.get_location())?;
                if !is_assignable(&assign.operand) {
                    return Err(ResolvingError::NotAssignable {
                        location: assign.operand.get_location(),
                    });
                }
                *assign.resolved_type.borrow_mut() = Some(operand_type);
            }
            Ast::Builtin(builtin) => match &builtin.kind {
                AstBuiltinKind::Type
                | AstBuiltinKind::Void
                | AstBuiltinKind::Bool
                | AstBuiltinKind::IntegerType { size: _, signed: _ } => {
                    *builtin.resolved_type.borrow_mut() = Some(get_or_add_type_type(type_cache))
                }
            },
        }
        ast.set_resolving(false);
        ast.get_type()
            .unwrap_or_else(|| panic!("type of ast should have been resolved"))
    })
}
