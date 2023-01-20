use std::collections::HashMap;
use std::fmt::Write;

use derive_more::Display;

use crate::{get_id, syntax::SourceSpan};

use super::{TypeID, Types};

#[derive(Clone, Debug, Display, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[display(fmt = "{_0}")]
pub struct BoundNodeID(usize);

fn slice_to_string<T: std::fmt::Display>(items: &[T]) -> Result<String, std::fmt::Error> {
    let mut result = String::new();
    write!(result, "[")?;
    for (i, item) in items.iter().enumerate() {
        if i > 0 {
            write!(result, ", ")?;
        }
        write!(result, "{item}")?;
    }
    write!(result, "]")?;
    Ok(result)
}

#[derive(Clone, Debug, Display, PartialEq, Eq)]
pub enum BoundNode {
    #[display(
        fmt = "ForwardDeclaration {{ resolved_node = {} }}",
        "_0.map_or(\"unresolved\".to_string(), |id| id.to_string())"
    )]
    ForwardDeclaration(Option<BoundNodeID>),
    #[display(
        fmt = "Procedure {{ parameters = {}, body = {body} }}",
        "slice_to_string(parameters).unwrap()"
    )]
    Procedure {
        parameters: Vec<BoundNodeID>,
        body: BoundNodeID,
    },
    #[display(fmt = "Parameter {{ index = {index} }}")]
    Parameter { index: usize },
    #[display(
        fmt = "Return {{ value = {} }}",
        "_0.map_or(\"no value\".to_string(), |id| id.to_string())"
    )]
    Return(Option<BoundNodeID>),
    #[display(fmt = "Block {{ expressions = {} }}", "slice_to_string(_0).unwrap()")]
    Block(Vec<BoundNodeID>),
    #[display(fmt = "Let {{ value = {value} }}")]
    Let { value: BoundNodeID },
    #[display(
        fmt = "Var {{ value = {} }}",
        "value.map_or(\"no value\".to_string(), |id| id.to_string())"
    )]
    Var { value: Option<BoundNodeID> },
    #[display(fmt = "Const {{ value = {value} }}")]
    Const { value: BoundNodeID },
    #[display(
        fmt = "Call {{ operand = {operand}, arguments = {} }}",
        "slice_to_string(arguments).unwrap()"
    )]
    Call {
        operand: BoundNodeID,
        arguments: Vec<BoundNodeID>,
    },
    #[display(fmt = "Type {{ typeid = {_0} }}")]
    Type(TypeID),
    #[display(fmt = "Name {{ resolved_node = {resolved_node} }}")]
    Name { resolved_node: BoundNodeID },
    #[display(fmt = "Integer {{ value = {_0} }}")]
    Integer(u128),
    #[display(fmt = "String {{ value = {_0} }}")]
    String(String),
}

impl BoundNode {
    pub fn is_constant(id: BoundNodeID, nodes: &BoundNodes) -> bool {
        let (node, _, _) = nodes.get_node(id);
        match *node {
            BoundNode::ForwardDeclaration(id) => {
                id.map_or(false, |id| Self::is_constant(id, nodes))
            }
            BoundNode::Procedure {
                parameters: _,
                body: _,
            } => true,
            BoundNode::Parameter { index: _ } => false,
            BoundNode::Return(value) => value.map_or(true, |value| Self::is_constant(value, nodes)),
            BoundNode::Block(_) => false,
            BoundNode::Let { value: _ } => false,
            BoundNode::Var { value: _ } => false,
            BoundNode::Const { value: _ } => true,
            BoundNode::Call {
                operand: _,
                arguments: _,
            } => false, // TODO: this should be allowed at some point
            BoundNode::Type(_) => true,
            BoundNode::Name { resolved_node } => Self::is_constant(resolved_node, nodes),
            BoundNode::Integer(_) => true,
            BoundNode::String(_) => true,
        }
    }
}

pub struct BoundNodes<'filepath> {
    types: Types<'filepath>,
    nodes: HashMap<BoundNodeID, (BoundNode, SourceSpan<'filepath>, TypeID)>,
}

impl<'filepath> BoundNodes<'filepath> {
    pub fn new() -> Self {
        Self {
            types: Types::new(),
            nodes: HashMap::new(),
        }
    }

    pub fn get_types(&self) -> &Types<'filepath> {
        &self.types
    }

    pub fn get_types_mut(&mut self) -> &mut Types<'filepath> {
        &mut self.types
    }

    pub fn add_node(
        &mut self,
        node: BoundNode,
        location: SourceSpan<'filepath>,
        typ: TypeID,
    ) -> BoundNodeID {
        let id = BoundNodeID(get_id());
        self.nodes.insert(id, (node, location, typ));
        id
    }

    pub fn get_node(&self, id: BoundNodeID) -> (&BoundNode, SourceSpan<'filepath>, TypeID) {
        let (node, location, id) = &self.nodes[&id];
        (node, *location, *id)
    }

    pub fn get_node_mut(
        &mut self,
        id: BoundNodeID,
    ) -> (&mut BoundNode, SourceSpan<'filepath>, TypeID) {
        let (node, location, id) = self.nodes.get_mut(&id).unwrap();
        (node, *location, *id)
    }
}

impl<'filepath> std::fmt::Display for BoundNodes<'filepath> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut nodes = self.nodes.iter().collect::<Vec<_>>();
        nodes.sort_by(|&(&a, _), &(&b, _)| a.cmp(&b));
        for (&id, &(ref node, _, typ)) in nodes {
            writeln!(f, "id = {id}, typeid = {typ}, node = {node}")?;
        }
        Ok(())
    }
}

impl<'filepath> Default for BoundNodes<'filepath> {
    fn default() -> Self {
        Self::new()
    }
}
