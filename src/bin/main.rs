use std::collections::HashMap;

use langite::*;

fn main() {
    let program = Ast::File(
        AstFile {
            resolving: false.into(),
            resolved_type: None.into(),
            expressions: vec![
                Ast::Call(
                    AstCall {
                        resolving: false.into(),
                        resolved_type: None.into(),
                        operand: Ast::Name(
                            AstName {
                                resolving: false.into(),
                                name: "do_whatever".into(),
                                resolved_declaration: None.into(),
                            }
                            .into(),
                        ),
                        arguments: vec![Ast::Name(
                            AstName {
                                resolving: false.into(),
                                name: "print_int".into(),
                                resolved_declaration: None.into(),
                            }
                            .into(),
                        )],
                    }
                    .into(),
                ),
                Ast::Procedure(
                    AstProcedure {
                        resolving: false.into(),
                        resolved_type: None.into(),
                        name: "print_int".into(),
                        parameters: vec![AstParameter {
                            resolving: false.into(),
                            resolved_type: None.into(),
                            name: "value".into(),
                            typ: Ast::Builtin(AstBuiltin::S64.into()),
                        }
                        .into()],
                        return_type: Ast::Builtin(AstBuiltin::Void.into()),
                        body: AstProcedureBody::ExternName("print_int".into()),
                    }
                    .into(),
                ),
                Ast::Procedure(
                    AstProcedure {
                        resolving: false.into(),
                        resolved_type: None.into(),
                        name: "do_whatever".into(),
                        parameters: vec![AstParameter {
                            resolving: false.into(),
                            resolved_type: None.into(),
                            name: "print_proc".into(),
                            typ: Ast::ProcedureType(
                                AstProcedureType {
                                    resolving: false.into(),
                                    resolved_type: None.into(),
                                    parameters: vec![AstParameter {
                                        resolving: false.into(),
                                        resolved_type: None.into(),
                                        name: "value".into(),
                                        typ: Ast::Builtin(AstBuiltin::S64.into()),
                                    }
                                    .into()],
                                    return_type: Ast::Builtin(AstBuiltin::Void.into()),
                                }
                                .into(),
                            ),
                        }
                        .into()],
                        return_type: Ast::Builtin(AstBuiltin::Void.into()),
                        body: AstProcedureBody::Scope(
                            AstScope {
                                resolving: false.into(),
                                resolved_type: None.into(),
                                expressions: vec![Ast::Call(
                                    AstCall {
                                        resolving: false.into(),
                                        resolved_type: None.into(),
                                        operand: Ast::Name(
                                            AstName {
                                                resolving: false.into(),
                                                name: "print_proc".into(),
                                                resolved_declaration: None.into(),
                                            }
                                            .into(),
                                        ),
                                        arguments: vec![Ast::Integer(
                                            AstInteger {
                                                resolving: false.into(),
                                                resolved_type: None.into(),
                                                value: 42,
                                            }
                                            .into(),
                                        )],
                                    }
                                    .into(),
                                )],
                            }
                            .into(),
                        ),
                    }
                    .into(),
                ),
            ],
        }
        .into(),
    );
    let mut names = HashMap::from([]);
    resolve_names(&program, &mut names).unwrap();
    resolve(&program, None, &mut vec![]).unwrap();
    let mut string = Vec::new();
    emit(&program, &mut 1, &mut string).unwrap();
    std::fs::write("output.c", &string).unwrap();
}
