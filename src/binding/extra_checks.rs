use derive_more::Display;

use crate::syntax::SourceSpan;

use super::{BoundNode, BoundNodeID, BoundNodes, Type};

#[derive(Debug, Display)]
pub enum ExtraChecksError<'filepath> {
    #[display(fmt = "{_0}: All control paths in a function/procedure must return a value")]
    DoesNotReturnInAllPaths(SourceSpan<'filepath>),
}

pub fn extra_checks<'filepath>(
    node: BoundNodeID,
    nodes: &BoundNodes<'filepath>,
) -> Result<(), ExtraChecksError<'filepath>> {
    let (node, location, _typ) = nodes.get_node(node);
    Ok(match *node {
        BoundNode::ForwardDeclaration(id) => {
            let id = id.expect("Internal Compiler Error: All forward declarations should be resolved at this point");
            extra_checks(id, nodes)?;
        }
        BoundNode::Procedure {
            ref parameters,
            body,
        } => {
            for &parameter in parameters {
                extra_checks(parameter, nodes)?;
            }
            extra_checks(body, nodes)?;
            if !returns_in_all_paths(body, nodes) {
                return Err(ExtraChecksError::DoesNotReturnInAllPaths(location));
            }
        }
        BoundNode::Parameter { index: _ } => (),
        BoundNode::Return(value) => {
            if let Some(value) = value {
                extra_checks(value, nodes)?;
            }
        }
        BoundNode::Block(ref expressions) => {
            for &expression in expressions {
                extra_checks(expression, nodes)?;
            }
        }
        BoundNode::Let { value } => {
            extra_checks(value, nodes)?;
        }
        BoundNode::Var { value } => {
            if let Some(value) = value {
                extra_checks(value, nodes)?;
            }
        }
        BoundNode::Const { value } => {
            extra_checks(value, nodes)?;
        }
        BoundNode::Call {
            operand,
            ref arguments,
        } => {
            extra_checks(operand, nodes)?;
            for &argument in arguments {
                extra_checks(argument, nodes)?;
            }
        }
        BoundNode::Type(_) => (),
        BoundNode::Name { resolved_node: _ } => (),
        BoundNode::Integer(_) => (),
        BoundNode::String(_) => (),
    })
}

fn returns_in_all_paths<'filepath>(node: BoundNodeID, nodes: &BoundNodes<'filepath>) -> bool {
    let (node, _, typ) = nodes.get_node(node);
    if matches!(nodes.get_types().get_type(typ), Type::Never) {
        return true;
    }
    match *node {
        BoundNode::ForwardDeclaration(id) => id.map_or(false, |id| returns_in_all_paths(id, nodes)),
        BoundNode::Procedure {
            parameters: _,
            body: _,
        } => false,
        BoundNode::Parameter { index: _ } => false,
        BoundNode::Return(_) => true,
        BoundNode::Block(ref expressions) => expressions
            .iter()
            .any(|&expression| returns_in_all_paths(expression, nodes)),
        BoundNode::Let { value } => returns_in_all_paths(value, nodes),
        BoundNode::Var { value } => value.map_or(false, |value| returns_in_all_paths(value, nodes)),
        BoundNode::Const { value } => returns_in_all_paths(value, nodes),
        BoundNode::Call {
            operand,
            ref arguments,
        } => {
            returns_in_all_paths(operand, nodes)
                || arguments
                    .iter()
                    .any(|&argument| returns_in_all_paths(argument, nodes))
        }
        BoundNode::Type(_) => false,
        BoundNode::Name { resolved_node: _ } => false,
        BoundNode::Integer(_) => false,
        BoundNode::String(_) => false,
    }
}
