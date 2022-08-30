use std::rc::Rc;

use crate::{Ast, AstParameter, AstProcedure, AstProcedureBody, Type};

const PREFIX: &'static str = "_";

fn emit_type(
    typ: &Type,
    name: Option<String>,
    stream: &mut dyn std::io::Write,
) -> Result<(), std::io::Error> {
    Ok(match typ {
        Type::Type => {
            write!(stream, "type")?;
            if let Some(name) = name {
                write!(stream, " {name}")?;
            }
        }
        Type::Void => {
            write!(stream, "Void")?;
            if let Some(name) = name {
                write!(stream, " {name}")?;
            }
        }
        Type::S64 => {
            write!(stream, "s64")?;
            if let Some(name) = name {
                write!(stream, " {name}")?;
            }
        }
        Type::Procedure {
            parameter_types,
            return_type,
        } => {
            emit_type(return_type, None, stream)?;
            if let Some(name) = name {
                write!(stream, " (*({name}))")?;
            } else {
                write!(stream, "(*)")?;
            }
            write!(stream, "(")?;
            if parameter_types.len() == 0 {
                write!(stream, "void")?;
            } else {
                for (i, parameter_type) in parameter_types.iter().enumerate() {
                    if i > 0 {
                        write!(stream, ", ")?;
                    }
                    emit_type(parameter_type, None, stream)?;
                }
            }
            write!(stream, ")")?;
        }
        Type::Pointer { pointed_to } => emit_type_ptr(pointed_to, name, stream)?,
    })
}

fn emit_type_ptr(
    typ: &Type,
    name: Option<String>,
    stream: &mut dyn std::io::Write,
) -> Result<(), std::io::Error> {
    emit_type(
        typ,
        format!("*{}", name.unwrap_or_else(|| "".into())).into(),
        stream,
    )
}

pub fn emit(
    ast: &Ast,
    next_id: &mut usize,
    stream: &mut dyn std::io::Write,
) -> Result<usize, std::io::Error> {
    Ok(match ast {
        Ast::File(file) => {
            write!(stream, "typedef unsigned long long type;\n")?;
            write!(stream, "typedef long long s64;\n")?;
            write!(
                stream,
                "_Static_assert(sizeof(s64) == 8, \"Expected s64 to be 8 bytes\");\n"
            )?;
            write!(stream, "\n")?;
            write!(stream, "typedef struct {{\n")?;
            write!(stream, "char buffer[1];\n")?;
            write!(stream, "}} Void;\n")?;
            write!(stream, "\n")?;
            {
                fn get_all_procedures(
                    ast: &Ast,
                    procedures: &mut Vec<Rc<AstProcedure>>,
                    walked: &mut Vec<Ast>,
                ) {
                    // TODO: is there a better way of doing this?
                    if !walked.contains(ast) {
                        walked.push(ast.clone());
                        match ast {
                            Ast::File(file) => {
                                for expression in &file.expressions {
                                    get_all_procedures(expression, procedures, walked);
                                }
                            }
                            Ast::Procedure(procedure) => {
                                procedures.push(procedure.clone());
                            }
                            Ast::ProcedureType(_) => todo!(),
                            Ast::Parameter(parameter) => {
                                get_all_procedures(&parameter.typ, procedures, walked);
                            }
                            Ast::Scope(scope) => {
                                for expression in &scope.expressions {
                                    get_all_procedures(expression, procedures, walked);
                                }
                            }
                            Ast::VarDeclaration(declaration) => {
                                get_all_procedures(&declaration.typ, procedures, walked);
                                get_all_procedures(&declaration.value, procedures, walked);
                            }
                            Ast::Name(name) => {
                                get_all_procedures(
                                    name.resolved_declaration.borrow().as_ref().unwrap(),
                                    procedures,
                                    walked,
                                );
                            }
                            Ast::Integer(_) => (),
                            Ast::Call(call) => {
                                get_all_procedures(&call.operand, procedures, walked);
                                for argument in &call.arguments {
                                    get_all_procedures(argument, procedures, walked);
                                }
                            }
                            Ast::Builtin(_) => (),
                        }
                    }
                }

                let mut procedures = vec![];
                get_all_procedures(ast, &mut procedures, &mut vec![]);

                fn emit_function_decl(
                    parameters: &[Rc<AstParameter>],
                    return_type: &Type,
                    name: &str,
                    stream: &mut dyn std::io::Write,
                ) -> Result<(), std::io::Error> {
                    emit_type(return_type, None, stream)?;
                    write!(stream, " {name}",)?;
                    write!(stream, "(")?;
                    if parameters.len() == 0 {
                        write!(stream, "void")?;
                    } else {
                        for (i, parameter) in parameters.iter().enumerate() {
                            if i > 0 {
                                write!(stream, ", ")?;
                            }
                            emit_type(
                                parameter.resolved_type.borrow().as_ref().unwrap(),
                                format!("_{}_{}", Rc::as_ptr(parameter) as usize, parameter.name)
                                    .into(),
                                stream,
                            )?;
                        }
                    }
                    write!(stream, ")")?;
                    Ok(())
                }

                // forward declare
                for procedure in &procedures {
                    match &procedure.body {
                        AstProcedureBody::ExternName(name) => {
                            let typ = procedure.resolved_type.borrow();
                            let typ = typ.as_ref().unwrap();
                            let return_type = typ.as_procedure().unwrap().1;
                            write!(stream, "static ")?;
                            emit_function_decl(
                                &procedure.parameters,
                                return_type,
                                &format!(
                                    "_impl_{}_{}",
                                    Rc::as_ptr(procedure) as usize,
                                    procedure.name
                                ),
                                stream,
                            )?;
                            write!(stream, " {{\nextern ")?;
                            emit_function_decl(&procedure.parameters, return_type, name, stream)?;
                            write!(stream, ";\n")?;
                            write!(stream, "return {name}(")?;
                            for (i, parameter) in procedure.parameters.iter().enumerate() {
                                if i > 0 {
                                    write!(stream, ", ")?;
                                }
                                write!(
                                    stream,
                                    "_{}_{}",
                                    Rc::as_ptr(parameter) as usize,
                                    parameter.name
                                )?;
                            }
                            write!(stream, ");\n}}\n")?;
                            write!(stream, "static ")?;
                            emit_type(
                                typ,
                                format!("_{}_{}", Rc::as_ptr(procedure) as usize, procedure.name)
                                    .into(),
                                stream,
                            )?;
                            write!(
                                stream,
                                " = &_impl_{}_{};\n",
                                Rc::as_ptr(procedure) as usize,
                                procedure.name
                            )?;
                        }
                        AstProcedureBody::Scope(_) => {
                            let typ = procedure.resolved_type.borrow();
                            let typ = typ.as_ref().unwrap();
                            let return_type = typ.as_procedure().unwrap().1;
                            let name =
                                format!("_{}_{}", Rc::as_ptr(procedure) as usize, procedure.name);
                            write!(stream, "static ")?;
                            emit_function_decl(
                                &procedure.parameters,
                                return_type,
                                &format!("_impl{name}"),
                                stream,
                            )?;
                            write!(stream, ";\n")?;
                            write!(stream, "static ")?;
                            emit_type(typ, name.clone().into(), stream)?;
                            write!(stream, " = &_impl{name};\n")?;
                        }
                    }
                }

                write!(stream, "\n")?;

                // implementation
                for procedure in &procedures {
                    match &procedure.body {
                        AstProcedureBody::ExternName(_) => (),
                        AstProcedureBody::Scope(scope) => {
                            let mut next_id = *next_id;
                            let typ = procedure.resolved_type.borrow();
                            let return_type = typ.as_ref().unwrap().as_procedure().unwrap().1;
                            write!(stream, "static ")?;
                            emit_function_decl(
                                &procedure.parameters,
                                return_type,
                                &format!(
                                    "_impl_{}_{}",
                                    Rc::as_ptr(procedure) as usize,
                                    procedure.name
                                ),
                                stream,
                            )?;
                            write!(stream, " {{\n")?;
                            emit(&Ast::Scope(scope.clone()), &mut next_id, stream)?;
                            if return_type.is_void() {
                                write!(stream, "return (Void){{}};\n")?;
                            }
                            write!(stream, "}}\n")?;
                        }
                    }
                }
            }
            {
                let mut next_id = *next_id;
                write!(stream, "\n")?;
                write!(stream, "int main() {{\n")?;
                for expression in &file.expressions {
                    emit(expression, &mut next_id, stream)?;
                }
                write!(stream, "return 0;\n")?;
                write!(stream, "}}\n")?;
                write!(stream, "\n")?;
                write!(stream, "#include <stdio.h>\n")?;
                write!(stream, "\n")?;
                write!(stream, "Void print_int(s64 value) {{\n")?;
                write!(stream, "printf(\"%lld\\n\", value);\n")?;
                write!(stream, "return (Void){{}};\n")?;
                write!(stream, "}}\n")?;
            }
            usize::MAX
        }
        Ast::Procedure(procedure) => match &procedure.body {
            AstProcedureBody::ExternName(_) => {
                let typ = procedure.resolved_type.borrow();
                let typ = typ.as_ref().unwrap();
                let id = *next_id;
                *next_id += 1;
                emit_type_ptr(typ, format!("{PREFIX}{id}").into(), stream)?;
                write!(
                    stream,
                    " = &_{}_{};\n",
                    Rc::as_ptr(procedure) as usize,
                    procedure.name
                )?;
                id
            }
            AstProcedureBody::Scope(_) => {
                let typ = procedure.resolved_type.borrow();
                let typ = typ.as_ref().unwrap();
                let id = *next_id;
                *next_id += 1;
                emit_type_ptr(typ, format!("{PREFIX}{id}").into(), stream)?;
                write!(
                    stream,
                    " = &_{}_{};\n",
                    Rc::as_ptr(procedure) as usize,
                    procedure.name
                )?;
                id
            }
        },
        Ast::ProcedureType(_) => todo!(),
        Ast::Parameter(_) => unreachable!(), // this is handled elsewhere
        Ast::Scope(scope) => {
            for expression in &scope.expressions {
                emit(expression, next_id, stream)?;
            }
            let id = *next_id;
            *next_id += 1;
            let typ = scope.resolved_type.borrow();
            let typ = typ.as_ref().unwrap();
            emit_type_ptr(typ, format!("{PREFIX}{id}").into(), stream)?;
            assert!(typ.is_void());
            write!(stream, " = &(Void){{}};\n")?;
            id
        }
        Ast::VarDeclaration(_) => todo!(),
        Ast::Name(name) => {
            let declaration = name.resolved_declaration.borrow();
            let declaration = declaration.as_ref().unwrap();
            let typ = declaration.get_type().unwrap();
            let id = *next_id;
            *next_id += 1;
            emit_type_ptr(&typ, format!("{PREFIX}{id}").into(), stream)?;
            write!(
                stream,
                " = &_{}_{};\n",
                declaration.get_ptr() as usize,
                name.name
            )?;
            id
        }
        Ast::Integer(integer) => {
            let id = *next_id;
            *next_id += 1;
            let typ = integer.resolved_type.borrow();
            let typ = typ.as_ref().unwrap();
            emit_type_ptr(typ, format!("{PREFIX}{id}").into(), stream)?;
            write!(stream, " = &(")?;
            emit_type(typ, None, stream)?;
            write!(stream, "){{{}}};\n", integer.value)?;
            id
        }
        Ast::Call(call) => {
            let operand = emit(&call.operand, next_id, stream)?;
            let arguments = call
                .arguments
                .iter()
                .map(|argument| emit(argument, next_id, stream))
                .collect::<Result<Vec<_>, _>>()?;
            let id = *next_id;
            *next_id += 1;
            emit_type(
                call.resolved_type.borrow().as_ref().unwrap(),
                format!("{PREFIX}{id}").into(),
                stream,
            )?;
            write!(stream, " = (*{PREFIX}{operand})(")?;
            for (i, argument) in arguments.iter().enumerate() {
                if i > 0 {
                    write!(stream, ", ")?;
                }
                write!(stream, "*{PREFIX}{argument}")?;
            }
            write!(stream, ");\n")?;
            id
        }
        Ast::Builtin(_) => todo!(),
    })
}
