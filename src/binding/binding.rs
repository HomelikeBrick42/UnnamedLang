use std::collections::HashMap;

use derive_more::Display;

use crate::syntax::{Ast, AstBody, GetLocation, SourceLocation, SourceSpan, Token, TokenKind};

use super::{BoundNode, BoundNodeID, BoundNodes, Type, TypeID, Types};

#[derive(Debug, Display)]
pub enum BindingError<'filepath, 'source> {
    #[display(
        fmt = "{location}: Redeclared name '{name}', previous declaration was here: '{previous_location}'"
    )]
    RedeclaredName {
        location: SourceSpan<'filepath>,
        previous_location: SourceSpan<'filepath>,
        name: &'source str,
    },
    #[display(fmt = "{location}: Undeclared name '{name}'")]
    UndeclaredName {
        location: SourceSpan<'filepath>,
        name: &'source str,
    },
    #[display(fmt = "{location}: Not a constant")]
    NotAConstant { location: SourceSpan<'filepath> },
    #[display(
        fmt = "{location}: Cyclic dependency detected (or you tried to use something before it was declared)"
    )]
    CyclicDependency { location: SourceSpan<'filepath> },
    #[display(fmt = "{location}: Expected {expected_type} but got {got_type}")]
    ExpectedType {
        location: SourceSpan<'filepath>,
        expected_type: String,
        got_type: String,
    },
    #[display(fmt = "{location}: Expected callable but got {got_type}")]
    ExpectedCallable {
        location: SourceSpan<'filepath>,
        got_type: String,
    },
    #[display(fmt = "{location}: Expected {expected} arguments but got {got} arguments")]
    ExpectedArgumentCount {
        location: SourceSpan<'filepath>,
        expected: usize,
        got: usize,
    },
    #[display(fmt = "{location}: `return` statement is not inside a procedure")]
    ReturnNotInsideProcedure { location: SourceSpan<'filepath> },
}

pub fn bind_file<'filepath, 'source>(
    filepath: &'filepath str,
    nodes: &mut BoundNodes<'filepath>,
    expressions: &[Ast<'filepath, 'source>],
    names: &HashMap<&'source str, BoundNodeID>,
) -> Result<BoundNodeID, BindingError<'filepath, 'source>> {
    let mut to_be_checked = vec![];
    let expressions = bind_block(nodes, expressions, names, None, &mut to_be_checked)?;

    // bodies of procedures or other things that need to be delayed
    while !to_be_checked.is_empty() {
        for (forward_declaration, mut names, expression, parent_procedure_return_type) in
            std::mem::take(&mut to_be_checked)
        {
            let bound_node = bind_expression(
                nodes,
                expression,
                &mut names,
                parent_procedure_return_type,
                &mut to_be_checked,
            )?;
            let BoundNode::ForwardDeclaration(id) = nodes.get_node_mut(forward_declaration).0 else { unreachable!() };
            *id = Some(bound_node);
            let (_, _, typ) = nodes.get_node(bound_node);
            let (_, _, forward_declared_type) = nodes.get_node(forward_declaration);
            let Type::ForwardDeclaration(id) = nodes.get_types_mut().get_type_mut(forward_declared_type) else { unreachable!() };
            *id = Some(typ);
        }
    }

    let void = nodes.get_types_mut().get_builtin_type(Type::Void);
    Ok(nodes.add_node(
        BoundNode::Block(expressions),
        SourceSpan {
            filepath,
            start: SourceLocation {
                position: 0,
                line: 1,
                column: 1,
            },
            end: SourceLocation {
                position: 0,
                line: 1,
                column: 1,
            },
        },
        void,
    ))
}

fn bind_block<'filepath, 'source, 'a>(
    nodes: &mut BoundNodes<'filepath>,
    expressions: &'a [Ast<'filepath, 'source>],
    names: &HashMap<&'source str, BoundNodeID>,
    parent_procedure_return_type: Option<TypeID>,
    to_be_checked: &mut Vec<(
        BoundNodeID,
        HashMap<&'source str, BoundNodeID>,
        &'a Ast<'filepath, 'source>,
        Option<TypeID>,
    )>,
) -> Result<Vec<BoundNodeID>, BindingError<'filepath, 'source>> {
    let mut names = names.clone();

    // forward declarations
    let mut forward_declarations = HashMap::new();
    for expression in expressions {
        let location = expression.get_location();
        match *expression {
            Ast::Procedure {
                name_token: Some(ref name_token),
                ..
            } => {
                let TokenKind::Name(name) = name_token.kind else { unreachable!() };
                let typ = nodes
                    .get_types_mut()
                    .add_type(Type::ForwardDeclaration(None), location);
                let forward_declaration =
                    nodes.add_node(BoundNode::ForwardDeclaration(None), location, typ);
                if let Some(previous_id) = names.insert(name, forward_declaration) {
                    let (_, previous_location, _) = nodes.get_node(previous_id);
                    return Err(BindingError::RedeclaredName {
                        location,
                        previous_location,
                        name,
                    });
                }
                forward_declarations.insert(name, forward_declaration);
            }
            _ => {}
        }
    }

    let mut bound_expressions = vec![];
    for expression in expressions {
        let id = bind_expression(
            nodes,
            expression,
            &mut names.clone(),
            parent_procedure_return_type,
            to_be_checked,
        )?;
        let (bound_expression, _, typ) = nodes.get_node(id);
        match bound_expression {
            BoundNode::Procedure {
                parameters: _,
                body: _,
            } => {
                let Ast::Procedure { name_token, .. } = expression else { unreachable!() };
                if let Some(name_token) = name_token {
                    let TokenKind::Name(name) = name_token.kind else { unreachable!() };
                    let (forward_declaration, _, declaration_type) =
                        nodes.get_node_mut(forward_declarations[name]);
                    if let BoundNode::ForwardDeclaration(declaration_id) = forward_declaration {
                        *declaration_id = Some(id);
                    } else {
                        unreachable!();
                    }
                    if let Type::ForwardDeclaration(declaration_type) =
                        nodes.get_types_mut().get_type_mut(declaration_type)
                    {
                        *declaration_type = Some(typ);
                    } else {
                        unreachable!();
                    }
                }
            }
            BoundNode::Let { value: _ } => {
                let Ast::Let { name_token: Token{ kind: TokenKind::Name(name), .. }, .. } = expression else { unreachable!() };
                if let Some(previous_id) = names.insert(name, id) {
                    let (_, location, _) = nodes.get_node(id);
                    let (_, previous_location, _) = nodes.get_node(previous_id);
                    return Err(BindingError::RedeclaredName {
                        location,
                        previous_location,
                        name,
                    });
                }
            }
            BoundNode::Var { value: _ } => {
                let Ast::Var { name_token: Token{ kind: TokenKind::Name(name), .. }, .. } = expression else { unreachable!() };
                if let Some(previous_id) = names.insert(name, id) {
                    let (_, location, _) = nodes.get_node(id);
                    let (_, previous_location, _) = nodes.get_node(previous_id);
                    return Err(BindingError::RedeclaredName {
                        location,
                        previous_location,
                        name,
                    });
                }
            }
            _ => {}
        }
        bound_expressions.push(id);
    }

    Ok(bound_expressions)
}

fn bind_expression<'filepath, 'source, 'a>(
    nodes: &mut BoundNodes<'filepath>,
    expression: &'a Ast<'filepath, 'source>,
    names: &mut HashMap<&'source str, BoundNodeID>,
    parent_procedure_return_type: Option<TypeID>,
    to_be_checked: &mut Vec<(
        BoundNodeID,
        HashMap<&'source str, BoundNodeID>,
        &'a Ast<'filepath, 'source>,
        Option<TypeID>,
    )>,
) -> Result<BoundNodeID, BindingError<'filepath, 'source>> {
    let location = expression.get_location();
    Ok(match *expression {
        Ast::Procedure {
            proc_token: _,
            ref name_token,
            open_parenthesis_token: _,
            ref parameters,
            close_parenthesis_token: _,
            fat_right_arrow_token: _,
            ref return_type,
            ref body,
        } => {
            let old_names = names;
            let mut names = old_names.clone();

            let parameters = parameters
                .iter()
                .enumerate()
                .map(|(index, parameter)| {
                    let location = parameter.get_location();
                    let TokenKind::Name(name) = parameter.name_token.kind else { unreachable!() };

                    let parameter_type = bind_expression(
                        nodes,
                        &parameter.typ,
                        &mut names,
                        parent_procedure_return_type,
                        to_be_checked,
                    )?;
                    if !BoundNode::is_constant(parameter_type, nodes) {
                        let (_, location, _) = nodes.get_node(parameter_type);
                        return Err(BindingError::NotAConstant { location });
                    }
                    let (_, _, parameter_type_type) = nodes.get_node(parameter_type);
                    let type_type = nodes.get_types_mut().get_builtin_type(Type::Type);
                    if parameter_type_type != type_type {
                        let mut expected_type = String::new();
                        Type::write(&mut expected_type, type_type, nodes.get_types()).unwrap();
                        let mut got_type = String::new();
                        Type::write(&mut got_type, parameter_type_type, nodes.get_types()).unwrap();
                        return Err(BindingError::ExpectedType {
                            location: parameter.get_location(),
                            expected_type,
                            got_type,
                        });
                    }
                    let parameter_type = eval_type(parameter_type, nodes, &names)?;

                    let param =
                        nodes.add_node(BoundNode::Parameter { index }, location, parameter_type);
                    let parameter_let =
                        nodes.add_node(BoundNode::Let { value: param }, location, parameter_type);
                    if let Some(old_id) = names.insert(name, parameter_let) {
                        let (_, old_location, _) = nodes.get_node(old_id);
                        return Err(BindingError::RedeclaredName {
                            location,
                            previous_location: old_location,
                            name,
                        });
                    }

                    Ok(parameter_let)
                })
                .collect::<Result<Vec<_>, _>>()?;

            let parameter_types = parameters
                .iter()
                .map(|&parameter| nodes.get_node(parameter).2)
                .collect::<Vec<_>>();

            let return_type = return_type
                .as_ref()
                .map(|return_type_ast| {
                    let return_type = bind_expression(
                        nodes,
                        return_type_ast,
                        &mut names,
                        parent_procedure_return_type,
                        to_be_checked,
                    )?;
                    if !BoundNode::is_constant(return_type, nodes) {
                        let (_, location, _) = nodes.get_node(return_type);
                        return Err(BindingError::NotAConstant { location });
                    }
                    let (_, _, return_type_type) = nodes.get_node(return_type);
                    let type_type = nodes.get_types_mut().get_builtin_type(Type::Type);
                    if return_type_type != type_type {
                        let mut expected_type = String::new();
                        Type::write(&mut expected_type, type_type, nodes.get_types()).unwrap();
                        let mut got_type = String::new();
                        Type::write(&mut got_type, return_type_type, nodes.get_types()).unwrap();
                        return Err(BindingError::ExpectedType {
                            location: return_type_ast.get_location(),
                            expected_type,
                            got_type,
                        });
                    }
                    let return_type = eval_type(return_type, nodes, &names)?;
                    Ok(return_type)
                })
                .transpose()?
                .unwrap_or_else(|| nodes.get_types_mut().get_builtin_type(Type::Void));

            let typ = nodes.get_types_mut().get_builtin_type(Type::Procedure {
                parameter_types,
                return_type,
            });

            if let Some(body) = body {
                let body_expression = match body {
                    AstBody::DoExpression {
                        do_token: _,
                        expression,
                    } => expression,
                    AstBody::Block(block) => block,
                };
                let body_type = nodes
                    .get_types_mut()
                    .add_type(Type::ForwardDeclaration(None), location);
                let body = nodes.add_node(BoundNode::ForwardDeclaration(None), location, body_type);
                to_be_checked.push((body, names, body_expression, Some(return_type)));

                let names = old_names;

                let id = nodes.add_node(BoundNode::Procedure { parameters, body }, location, typ);
                if let Some(name_token) = name_token {
                    let TokenKind::Name(name) = name_token.kind else { unreachable!() };
                    if let Some(old_decl) = names.insert(name, id) {
                        let (old_decl, old_location, _) = nodes.get_node(old_decl);
                        if !matches!(old_decl, BoundNode::ForwardDeclaration(_)) {
                            return Err(BindingError::RedeclaredName {
                                location,
                                previous_location: old_location,
                                name,
                            });
                        }
                    }
                }
                id
            } else {
                assert!(name_token.is_none()); // a procedure type cannot have a name
                nodes.add_node(BoundNode::Type(typ), location, typ)
            }
        }
        Ast::Return {
            ref return_token,
            ref value,
        } => {
            let Some(return_type) = parent_procedure_return_type else {
                return Err(BindingError::ReturnNotInsideProcedure {
                    location: return_token.get_location(),
                });
            };

            let value = value
                .as_ref()
                .map(|value| {
                    let value = bind_expression(
                        nodes,
                        value,
                        names,
                        parent_procedure_return_type,
                        to_be_checked,
                    )?;
                    Ok(value)
                })
                .transpose()?;

            let value_type = value.map_or(
                nodes.get_types_mut().get_builtin_type(Type::Void),
                |value| nodes.get_node(value).2,
            );

            if value_type != return_type {
                let mut expected_type = String::new();
                Type::write(&mut expected_type, return_type, nodes.get_types()).unwrap();
                let mut got_type = String::new();
                Type::write(&mut got_type, value_type, nodes.get_types()).unwrap();
                return Err(BindingError::ExpectedType {
                    location: return_token.get_location(),
                    expected_type,
                    got_type,
                });
            }

            let never = nodes.get_types_mut().get_builtin_type(Type::Never);
            nodes.add_node(BoundNode::Return(value), location, never)
        }
        Ast::Block {
            open_brace_token: _,
            ref expressions,
            close_brace_token: _,
        } => {
            let expressions = bind_block(
                nodes,
                expressions,
                names,
                parent_procedure_return_type,
                to_be_checked,
            )?;
            let void = nodes.get_types_mut().get_builtin_type(Type::Void);
            nodes.add_node(BoundNode::Block(expressions), location, void)
        }
        Ast::Let {
            let_token: _,
            name_token: _,
            colon_token: _,
            typ: _,
            value: _,
        } => todo!(),
        Ast::Var {
            var_token: _,
            name_token: _,
            colon_token: _,
            typ: _,
            value: _,
        } => todo!(),
        Ast::Unary {
            operator: _,
            operand: _,
        } => todo!(),
        Ast::Binary {
            left: _,
            operator: _,
            right: _,
        } => todo!(),
        Ast::Call {
            ref operand,
            ref open_parenthesis_token,
            ref arguments,
            close_parenthesis_token: _,
        } => {
            let operand = bind_expression(
                nodes,
                operand,
                names,
                parent_procedure_return_type,
                to_be_checked,
            )?;
            let (_, operand_location, operand_type) = nodes.get_node(operand);
            let Type::Procedure {
                parameter_types,
                return_type,
            } = get_type(operand_location, operand_type, nodes.get_types())?.clone() else {
                let mut type_str = String::new();
                Type::write(&mut type_str, operand_type, nodes.get_types()).unwrap();
                return Err(BindingError::ExpectedCallable {
                    location: operand_location,
                    got_type: type_str,
                });
            };

            if arguments.len() != parameter_types.len() {
                return Err(BindingError::ExpectedArgumentCount {
                    location: open_parenthesis_token.get_location(),
                    expected: parameter_types.len(),
                    got: arguments.len(),
                });
            }
            let arguments = arguments
                .iter()
                .enumerate()
                .map(|(i, argument)| {
                    let argument = bind_expression(
                        nodes,
                        argument,
                        names,
                        parent_procedure_return_type,
                        to_be_checked,
                    )?;
                    let (_, argument_location, argument_type) = nodes.get_node(argument);
                    if argument_type != parameter_types[i] {
                        let mut expected_type = String::new();
                        Type::write(&mut expected_type, parameter_types[i], nodes.get_types())
                            .unwrap();
                        let mut got_type = String::new();
                        Type::write(&mut got_type, argument_type, nodes.get_types()).unwrap();
                        return Err(BindingError::ExpectedType {
                            location: argument_location,
                            expected_type,
                            got_type,
                        });
                    }
                    Ok(argument)
                })
                .collect::<Result<Vec<_>, _>>()?;

            nodes.add_node(
                BoundNode::Call { operand, arguments },
                location,
                return_type,
            )
        }
        Ast::Name { ref name_token } => {
            let TokenKind::Name(name) = name_token.kind else { unreachable!() };
            let Some(&resolved_node) = names.get(name) else {
                return Err(BindingError::UndeclaredName { location, name });
            };
            let (_, _, typ) = nodes.get_node(resolved_node);
            nodes.add_node(BoundNode::Name { resolved_node }, location, typ)
        }
        Ast::Integer { ref integer_token } => {
            let TokenKind::Integer(value) = integer_token.kind else { unreachable!() };
            let int = nodes.get_types_mut().get_builtin_type(Type::Int);
            nodes.add_node(BoundNode::Integer(value), location, int)
        }
        Ast::String { ref string_token } => {
            let TokenKind::String(ref value) = string_token.kind else { unreachable!() };
            let char = nodes.get_types_mut().get_builtin_type(Type::Char);
            let string = nodes
                .get_types_mut()
                .get_builtin_type(Type::Slice { inner_type: char });
            nodes.add_node(BoundNode::String(value.clone()), location, string)
        }
    })
}

fn get_type<'filepath, 'source, 't>(
    location: SourceSpan<'filepath>,
    typ: TypeID,
    types: &'t Types,
) -> Result<&'t Type, BindingError<'filepath, 'source>> {
    let typ = types.get_type(typ);
    if let Type::ForwardDeclaration(typ) = *typ {
        if let Some(typ) = typ {
            get_type(location, typ, types)
        } else {
            Err(BindingError::CyclicDependency { location })
        }
    } else {
        Ok(typ)
    }
}

fn eval_type<'filepath, 'source>(
    node: BoundNodeID,
    nodes: &BoundNodes<'filepath>,
    names: &HashMap<&'source str, BoundNodeID>,
) -> Result<TypeID, BindingError<'filepath, 'source>> {
    let (node, location, _) = nodes.get_node(node);
    Ok(match *node {
        BoundNode::ForwardDeclaration(node) => eval_type(
            node.ok_or(BindingError::CyclicDependency { location })?,
            nodes,
            names,
        )?,
        BoundNode::Procedure {
            parameters: _,
            body: _,
        } => unreachable!(),
        BoundNode::Parameter { index: _ } => unreachable!(),
        BoundNode::Return(_) => unreachable!(),
        BoundNode::Block(_) => unreachable!(),
        BoundNode::Let { value: _ } => unreachable!(),
        BoundNode::Var { value: _ } => unreachable!(),
        BoundNode::Const { value } => eval_type(value, nodes, names)?,
        BoundNode::Call {
            operand: _,
            arguments: _,
        } => unreachable!(),
        BoundNode::Type(typ) => typ,
        BoundNode::Name { resolved_node } => eval_type(resolved_node, nodes, names)?,
        BoundNode::Integer(_) => unreachable!(),
        BoundNode::String(_) => unreachable!(),
    })
}
