use std::collections::HashMap;

use derive_more::Display;

use crate::{get_id, syntax::SourceSpan};

use super::{TypeID, Types};

#[derive(Clone, Debug, Display, Copy, PartialEq, Eq, Hash)]
#[display(fmt = "{_0}")]
pub struct BoundNodeID(usize);

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum BoundNode {
    Procedure {
        parameters: Vec<BoundNodeID>,
        body: BoundNodeID,
    },
    Return(BoundNodeID),
    Block(Vec<BoundNodeID>),
    Let {
        value: BoundNodeID,
    },
    Var {
        value: Option<BoundNodeID>,
    },
    Call {
        operand: BoundNodeID,
        arguments: Vec<BoundNodeID>,
    },
    ProcedureType(TypeID),
    Name {
        resolved_node: BoundNodeID,
    },
    Integer(u128),
    String(String),
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

impl<'filepath> Default for BoundNodes<'filepath> {
    fn default() -> Self {
        Self::new()
    }
}
