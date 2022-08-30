use std::{collections::HashMap, rc::Rc};

use derive_more::{Display, IsVariant};
use enum_as_inner::EnumAsInner;

use crate::{Ast, AstBuiltin, AstParameter, AstProcedure, AstProcedureBody, AstVar, Type};

#[derive(Clone, Debug, Display, PartialEq, IsVariant, EnumAsInner)]
pub enum ResolvingError {
    #[display(fmt = "Redeclaration of '{name}'")]
    Redeclaration { name: String },
    #[display(fmt = "'{name}' is undeclared")]
    UndeclaredName { name: String },
    #[display(fmt = "Expected type '{expected}', but got type '{got}'")]
    ExpectedType { expected: Rc<Type>, got: Rc<Type> },
}

#[derive(Clone, Debug, PartialEq, IsVariant, EnumAsInner)]
pub enum Declaration {
    Procedure(Rc<AstProcedure>),
    Parameter(Rc<AstParameter>),
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
            Declaration::Var(declaration) => Ast::VarDeclaration(declaration.clone()),
            Declaration::Builtin(builtin) => Ast::Builtin(builtin.clone()),
        }
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
                    if let Some(_) = names.insert(
                        procedure.name.clone(),
                        Declaration::Procedure(procedure.clone()),
                    ) {
                        return Err(ResolvingError::Redeclaration {
                            name: procedure.name.clone(),
                        });
                    }
                }
                _ => (),
            }
        }
        for expression in expressions {
            resolve_names(expression, names)?;
        }
        Ok(())
    }

    Ok(match ast {
        Ast::File(file) => {
            scope_like(&file.expressions, &mut names.clone())?;
        }
        Ast::Procedure(procedure) => {
            assert!(names.contains_key(&procedure.name));
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
            if let Some(_) = names.insert(
                parameter.name.clone(),
                Declaration::Parameter(parameter.clone()),
            ) {
                return Err(ResolvingError::Redeclaration {
                    name: parameter.name.clone(),
                });
            }
        }
        Ast::Scope(scope) => {
            scope_like(&scope.expressions, &mut names.clone())?;
        }
        Ast::VarDeclaration(declaration) => {
            resolve_names(&declaration.typ, names)?;
            resolve_names(&declaration.value, names)?;
            if let Some(_) = names.insert(
                declaration.name.clone(),
                Declaration::Var(declaration.clone()),
            ) {
                return Err(ResolvingError::Redeclaration {
                    name: declaration.name.clone(),
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
        Ast::Builtin(builtin) => match builtin.as_ref() {
            AstBuiltin::Type => (),
            AstBuiltin::Void => (),
            AstBuiltin::S64 => (),
        },
    })
}

fn expect_type(typ: &Rc<Type>, expected: &Rc<Type>) -> Result<(), ResolvingError> {
    if typ == expected {
        Ok(())
    } else {
        Err(ResolvingError::ExpectedType {
            expected: expected.clone(),
            got: typ.clone(),
        })
    }
}

fn eval_type(ast: &Ast) -> Result<Rc<Type>, ResolvingError> {
    Ok(match ast {
        Ast::File(_) => todo!(),
        Ast::Procedure(_) => todo!(),
        Ast::ProcedureType(procedure_type) => Type::Procedure {
            parameter_types: procedure_type
                .parameter_types
                .iter()
                .map(eval_type)
                .collect::<Result<_, _>>()?,
            return_type: eval_type(&procedure_type.return_type)?,
        }
        .into(),
        Ast::Parameter(_) => todo!(),
        Ast::Scope(_) => todo!(),
        Ast::VarDeclaration(_) => todo!(),
        Ast::Name(name) => eval_type(name.resolved_declaration.borrow().as_ref().unwrap())?,
        Ast::Integer(_) => todo!(),
        Ast::Call(_) => todo!(),
        Ast::Builtin(builtin) => match builtin.as_ref() {
            AstBuiltin::Type => Type::Type.into(),
            AstBuiltin::Void => Type::Void.into(),
            AstBuiltin::S64 => Type::S64.into(),
        },
    })
}

pub fn resolve(
    ast: &Ast,
    suggested_type: Option<Rc<Type>>,
    defered_asts: &mut Vec<Ast>,
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
                *file.resolved_type.borrow_mut() = Some(Type::Void.into());
                for expression in &file.expressions {
                    resolve(expression, None, defered_asts)?;
                }
                while let Some(ast) = defered_asts.pop() {
                    resolve(&ast, None, defered_asts)?;
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
                        suggested_proc_type.map(|(parameters, _)| parameters[i].clone());
                    parameter_types.push(resolve(
                        &Ast::Parameter(parameter.clone()),
                        suggested_parameter_type,
                        defered_asts,
                    )?);
                }
                let return_type_type = resolve(
                    &procedure.return_type,
                    Some(Type::Type.into()),
                    defered_asts,
                )?;
                expect_type(&return_type_type, &Type::Type.into())?;
                *procedure.resolved_type.borrow_mut() = Some(
                    Type::Procedure {
                        parameter_types,
                        return_type: eval_type(&procedure.return_type)?,
                    }
                    .into(),
                );
                match &procedure.body {
                    AstProcedureBody::ExternName(_) => (),
                    AstProcedureBody::Scope(scope) => defered_asts.push(Ast::Scope(scope.clone())),
                }
            }
            Ast::ProcedureType(procedure_type) => {
                *procedure_type.resolved_type.borrow_mut() = Some(Type::Type.into());
                for parameter in &procedure_type.parameter_types {
                    resolve(&parameter, None, defered_asts)?;
                }
                let return_type_type = resolve(
                    &procedure_type.return_type,
                    Some(Type::Type.into()),
                    defered_asts,
                )?;
                expect_type(&return_type_type, &Type::Type.into())?;
            }
            Ast::Parameter(parameter) => {
                let type_type = resolve(&parameter.typ, Some(Type::Type.into()), defered_asts)?;
                expect_type(&type_type, &Type::Type.into())?;
                *parameter.resolved_type.borrow_mut() = Some(eval_type(&parameter.typ)?);
            }
            Ast::Scope(scope) => {
                *scope.resolved_type.borrow_mut() = Some(Type::Void.into());
                for expression in &scope.expressions {
                    resolve(expression, None, defered_asts)?;
                }
            }
            Ast::VarDeclaration(declaration) => {
                let type_type = resolve(&declaration.typ, Some(Type::Type.into()), defered_asts)?;
                expect_type(&type_type, &Type::Type.into())?;
                let resolved_type = eval_type(&declaration.typ)?;
                *declaration.resolved_type.borrow_mut() = Some(resolved_type.clone());
                let value_type = resolve(
                    &declaration.value,
                    declaration.resolved_type.borrow().clone(),
                    defered_asts,
                )?;
                expect_type(&value_type, &resolved_type)?;
            }
            Ast::Name(name) => {
                let declaration = name.resolved_declaration.borrow();
                let declaration = declaration
                    .as_ref()
                    .expect("the name should be resolved at this point");
                resolve(&declaration, suggested_type, defered_asts)?;
            }
            Ast::Integer(integer) => {
                if integer.value >= i64::MAX as _ {
                    todo!("error")
                } else {
                    *integer.resolved_type.borrow_mut() = Some(Type::S64.into());
                }
            }
            Ast::Call(call) => {
                let operand_type = resolve(&call.operand, None, defered_asts)?; // TODO: is there some way we can expect the type here?
                let (parameter_types, return_type) =
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
                    let argument_type =
                        resolve(argument, Some(expected_argument_type.clone()), defered_asts)?;
                    expect_type(&argument_type, expected_argument_type)?;
                }
            }
            Ast::Builtin(builtin) => match builtin.as_ref() {
                AstBuiltin::Type => (),
                AstBuiltin::Void => (),
                AstBuiltin::S64 => (),
            },
        }
        ast.set_resolving(false);
        ast.get_type()
            .unwrap_or_else(|| panic!("type of ast should have been resolved",))
    })
}
