use std::rc::Rc;

use crate::{
    Ast, AstAssignDirection, AstParameter, AstProcedure, AstProcedureBody, BinaryOperator, Type,
};

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
        Type::Bool => {
            write!(stream, "_Bool")?;
            if let Some(name) = name {
                write!(stream, " {name}")?;
            }
        }
        Type::Integer { size, signed } => {
            write!(stream, "{}{}", if *signed { "s" } else { "u" }, size * 8)?;
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
            write!(stream, "typedef signed char s8;\n")?;
            write!(
                stream,
                "_Static_assert(sizeof(s8) == 1, \"Expected s8 to be 1 byte\");\n"
            )?;
            write!(stream, "typedef signed short s16;\n")?;
            write!(
                stream,
                "_Static_assert(sizeof(s16) == 2, \"Expected s16 to be 2 bytes\");\n"
            )?;
            write!(stream, "typedef signed int s32;\n")?;
            write!(
                stream,
                "_Static_assert(sizeof(s32) == 4, \"Expected s32 to be 4 bytes\");\n"
            )?;
            write!(stream, "typedef signed long long s64;\n")?;
            write!(
                stream,
                "_Static_assert(sizeof(s64) == 8, \"Expected s64 to be 8 bytes\");\n"
            )?;
            write!(stream, "typedef unsigned char u8;\n")?;
            write!(
                stream,
                "_Static_assert(sizeof(u8) == 1, \"Expected u8 to be 1 byte\");\n"
            )?;
            write!(stream, "typedef unsigned short u16;\n")?;
            write!(
                stream,
                "_Static_assert(sizeof(u16) == 2, \"Expected u16 to be 2 bytes\");\n"
            )?;
            write!(stream, "typedef unsigned int u32;\n")?;
            write!(
                stream,
                "_Static_assert(sizeof(u32) == 4, \"Expected u32 to be 4 bytes\");\n"
            )?;
            write!(stream, "typedef unsigned long long u64;\n")?;
            write!(
                stream,
                "_Static_assert(sizeof(u64) == 8, \"Expected u64 to be 8 bytes\");\n"
            )?;
            write!(stream, "\n")?;
            write!(stream, "typedef struct {{\n")?;
            write!(stream, "char buffer[0];\n")?;
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
                                for parameter in &procedure.parameters {
                                    get_all_procedures(
                                        &Ast::Parameter(parameter.clone()),
                                        procedures,
                                        walked,
                                    );
                                }
                                get_all_procedures(&procedure.return_type, procedures, walked);
                                match &procedure.body {
                                    AstProcedureBody::ExternName(_) => (),
                                    AstProcedureBody::Scope(scope) => get_all_procedures(
                                        &Ast::Scope(scope.clone()),
                                        procedures,
                                        walked,
                                    ),
                                }
                            }
                            Ast::ProcedureType(procedure_type) => {
                                for parameter in &procedure_type.parameter_types {
                                    get_all_procedures(parameter, procedures, walked);
                                }
                                get_all_procedures(&procedure_type.return_type, procedures, walked);
                            }
                            Ast::Parameter(parameter) => {
                                get_all_procedures(&parameter.typ, procedures, walked);
                            }
                            Ast::Scope(scope) => {
                                for expression in &scope.expressions {
                                    get_all_procedures(expression, procedures, walked);
                                }
                            }
                            Ast::LetDeclaration(declaration) => {
                                if let Some(typ) = &declaration.typ {
                                    get_all_procedures(typ, procedures, walked);
                                }
                                get_all_procedures(&declaration.value, procedures, walked);
                            }
                            Ast::VarDeclaration(declaration) => {
                                if let Some(typ) = &declaration.typ {
                                    get_all_procedures(typ, procedures, walked);
                                }
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
                            Ast::Return(returnn) => {
                                if let Some(value) = &returnn.value {
                                    get_all_procedures(value, procedures, walked);
                                }
                            }
                            Ast::Binary(binary) => {
                                get_all_procedures(&binary.left, procedures, walked);
                                get_all_procedures(&binary.right, procedures, walked);
                            }
                            Ast::If(iff) => {
                                get_all_procedures(&iff.condition, procedures, walked);
                                get_all_procedures(&iff.then_expression, procedures, walked);
                                if let Some(else_expression) = &iff.else_expression {
                                    get_all_procedures(else_expression, procedures, walked);
                                }
                            }
                            Ast::While(whilee) => {
                                get_all_procedures(&whilee.condition, procedures, walked);
                                get_all_procedures(&whilee.then_expression, procedures, walked);
                            }
                            Ast::Cast(cast) => {
                                get_all_procedures(&cast.typ, procedures, walked);
                                get_all_procedures(&cast.operand, procedures, walked);
                            }
                            Ast::Assign(assign) => match &assign.direction {
                                AstAssignDirection::Left => {
                                    get_all_procedures(&assign.operand, procedures, walked);
                                    get_all_procedures(&assign.value, procedures, walked);
                                }
                                AstAssignDirection::Right => {
                                    get_all_procedures(&assign.value, procedures, walked);
                                    get_all_procedures(&assign.operand, procedures, walked);
                                }
                            },
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
                            write!(stream, "extern ")?;
                            emit_function_decl(&procedure.parameters, return_type, name, stream)?;
                            write!(stream, ";\n")?;
                            write!(stream, "static ")?;
                            emit_type(
                                typ,
                                format!("_{}_{}", Rc::as_ptr(procedure) as usize, procedure.name)
                                    .into(),
                                stream,
                            )?;
                            write!(stream, " = &{};\n", name)?;
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
                write!(stream, "Void print_s64(s64 value) {{\n")?;
                write!(stream, "extern int printf(const char *, ...);\n")?;
                write!(stream, "printf(\"%lld\\n\", value);\n")?;
                write!(stream, "return (Void){{}};\n")?;
                write!(stream, "}}\n")?;
                write!(stream, "Void print_u64(u64 value) {{\n")?;
                write!(stream, "extern int printf(const char *, ...);\n")?;
                write!(stream, "printf(\"%llu\\n\", value);\n")?;
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
        Ast::LetDeclaration(declaration) => {
            let typ = declaration.resolved_type.borrow();
            let typ = typ.as_ref().unwrap();
            let value = emit(&declaration.value, next_id, stream)?;
            let id = *next_id;
            *next_id += 1;
            let name = format!("_{}_{}", Rc::as_ptr(declaration) as usize, declaration.name);
            emit_type(&typ, name.clone().into(), stream)?;
            write!(stream, " = {PREFIX}{value};\n")?;
            emit_type_ptr(&typ, format!("{PREFIX}{id}").into(), stream)?;
            write!(stream, " = &{name};\n")?;
            id
        }
        Ast::VarDeclaration(declaration) => {
            let typ = declaration.resolved_type.borrow();
            let typ = typ.as_ref().unwrap();
            let value = emit(&declaration.value, next_id, stream)?;
            let id = *next_id;
            *next_id += 1;
            let name = format!("_{}_{}", Rc::as_ptr(declaration) as usize, declaration.name);
            emit_type(&typ, name.clone().into(), stream)?;
            write!(stream, " = *{PREFIX}{value};\n")?;
            emit_type_ptr(&typ, format!("{PREFIX}{id}").into(), stream)?;
            write!(stream, " = &{name};\n")?;
            id
        }
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
            let return_id = *next_id;
            *next_id += 1;
            emit_type(
                call.resolved_type.borrow().as_ref().unwrap(),
                format!("{PREFIX}{return_id}").into(),
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
            let id = *next_id;
            *next_id += 1;
            emit_type_ptr(
                call.resolved_type.borrow().as_ref().unwrap(),
                format!("{PREFIX}{id}").into(),
                stream,
            )?;
            write!(stream, " = &{PREFIX}{return_id};\n")?;
            id
        }
        Ast::Return(returnn) => {
            if let Some(value) = &returnn.value {
                let value_id = emit(value, next_id, stream)?;
                write!(stream, "return *{PREFIX}{value_id};\n")?;
            } else {
                write!(stream, "return (Void){{}};\n")?;
            }
            let id = *next_id;
            *next_id += 1;
            let typ = returnn.resolved_type.borrow();
            let typ = typ.as_ref().unwrap();
            emit_type_ptr(typ, format!("{PREFIX}{id}").into(), stream)?;
            assert!(typ.is_void());
            write!(stream, " = &(Void){{}};\n")?;
            id
        }
        Ast::Binary(binary) => {
            let left = emit(&binary.left, next_id, stream)?;
            let right = emit(&binary.right, next_id, stream)?;
            let id = *next_id;
            *next_id += 1;
            let typ = binary.resolved_type.borrow();
            let typ = typ.as_ref().unwrap();
            emit_type_ptr(typ, format!("{PREFIX}{id}").into(), stream)?;
            write!(stream, " = &(")?;
            emit_type(typ, None, stream)?;
            write!(stream, "){{")?;
            assert!(binary.left.get_type().unwrap().as_integer().is_some());
            assert!(binary.right.get_type().unwrap().as_integer().is_some());
            match &binary.operator {
                BinaryOperator::Add => write!(stream, "*{PREFIX}{left} + *{PREFIX}{right}")?,
                BinaryOperator::Subtract => write!(stream, "*{PREFIX}{left} - *{PREFIX}{right}")?,
                BinaryOperator::Multiply => write!(stream, "*{PREFIX}{left} * *{PREFIX}{right}")?,
                BinaryOperator::Divide => write!(stream, "*{PREFIX}{left} / *{PREFIX}{right}")?,
                BinaryOperator::Equal => write!(stream, "*{PREFIX}{left} == *{PREFIX}{right}")?,
                BinaryOperator::NotEqual => write!(stream, "*{PREFIX}{left} != *{PREFIX}{right}")?,
                BinaryOperator::LessThan => write!(stream, "*{PREFIX}{left} < *{PREFIX}{right}")?,
                BinaryOperator::GreaterThan => {
                    write!(stream, "*{PREFIX}{left} > *{PREFIX}{right}")?
                }
                BinaryOperator::LessThanEqual => {
                    write!(stream, "*{PREFIX}{left} <= *{PREFIX}{right}")?
                }
                BinaryOperator::GreaterThanEqual => {
                    write!(stream, "*{PREFIX}{left} >= *{PREFIX}{right}")?
                }
            }
            write!(stream, "}};\n")?;
            id
        }
        Ast::If(iff) => {
            let typ = iff.resolved_type.borrow();
            let typ = typ.as_ref().unwrap();
            let condition = emit(&iff.condition, next_id, stream)?;
            let else_id = *next_id;
            *next_id += 1;
            let id = *next_id;
            *next_id += 1;
            emit_type_ptr(typ, format!("{PREFIX}{id}").into(), stream)?;
            assert!(typ.is_void());
            write!(stream, " = &(Void){{}};\n")?;
            write!(
                stream,
                "if (!*{PREFIX}{condition}) goto {PREFIX}{else_id};\n"
            )?;
            let then_expression = emit(&iff.then_expression, next_id, stream)?;
            write!(stream, "{PREFIX}{id} = {PREFIX}{then_expression};\n")?;
            let end_id = *next_id;
            *next_id += 1;
            write!(stream, "goto {PREFIX}{end_id};\n")?;
            write!(stream, "{PREFIX}{else_id}:;\n")?;
            if let Some(else_expression) = &iff.else_expression {
                let else_expression = emit(else_expression, next_id, stream)?;
                write!(stream, "{PREFIX}{id} = {PREFIX}{else_expression};\n")?;
            }
            write!(stream, "{PREFIX}{end_id}:;\n")?;
            id
        }
        Ast::While(whilee) => {
            let typ = whilee.resolved_type.borrow();
            let typ = typ.as_ref().unwrap();
            let start_id = *next_id;
            *next_id += 1;
            write!(stream, "{PREFIX}{start_id}:;\n")?;
            let condition = emit(&whilee.condition, next_id, stream)?;
            let id = *next_id;
            *next_id += 1;
            emit_type_ptr(typ, format!("{PREFIX}{id}").into(), stream)?;
            assert!(typ.is_void());
            write!(stream, " = &(Void){{}};\n")?;
            let end_id = *next_id;
            *next_id += 1;
            write!(
                stream,
                "if (!*{PREFIX}{condition}) goto {PREFIX}{end_id};\n"
            )?;
            let then_expression = emit(&whilee.then_expression, next_id, stream)?;
            write!(stream, "{PREFIX}{id} = {PREFIX}{then_expression};\n")?;
            write!(stream, "goto {PREFIX}{start_id};\n")?;
            write!(stream, "{PREFIX}{end_id}:;\n")?;
            id
        }
        Ast::Cast(cast) => {
            let operand = emit(&cast.operand, next_id, stream)?;
            let typ = cast.resolved_type.borrow();
            let typ = typ.as_ref().unwrap();
            if &cast.operand.get_type().unwrap() == typ {
                operand
            } else {
                assert!(typ.as_integer().is_some());
                assert!(cast.operand.get_type().unwrap().as_integer().is_some());
                let id = *next_id;
                *next_id += 1;
                emit_type_ptr(&typ, format!("{PREFIX}{id}").into(), stream)?;
                write!(stream, " = &(")?;
                emit_type(&typ, None, stream)?;
                write!(stream, "){{(")?;
                emit_type(&typ, None, stream)?;
                write!(stream, ")*{PREFIX}{operand}}};\n")?;
                id
            }
        }
        Ast::Assign(assign) => match &assign.direction {
            AstAssignDirection::Left => {
                let operand = emit(&assign.operand, next_id, stream)?;
                let value = emit(&assign.value, next_id, stream)?;
                write!(stream, "*{PREFIX}{operand} = *{PREFIX}{value};\n")?;
                operand
            }
            AstAssignDirection::Right => {
                let value = emit(&assign.value, next_id, stream)?;
                let operand = emit(&assign.operand, next_id, stream)?;
                write!(stream, "*{PREFIX}{operand} = *{PREFIX}{value};\n")?;
                operand
            }
        },
        Ast::Builtin(_) => todo!(),
    })
}
