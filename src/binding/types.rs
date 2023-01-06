use std::collections::HashMap;

use derive_more::Display;

use crate::{get_id, syntax::SourceSpan};

#[derive(Clone, Debug, Display, Copy, PartialEq, Eq, Hash)]
#[display(fmt = "{_0}")]
pub struct TypeID(usize);

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Type {
    Never,
    Int,
    Procedure {
        parameter_types: Vec<TypeID>,
        return_type: TypeID,
    },
}

pub struct Types<'filepath> {
    types: HashMap<TypeID, (Type, SourceSpan<'filepath>)>,
}

impl<'filepath> Types<'filepath> {
    pub fn new() -> Self {
        Self {
            types: HashMap::new(),
        }
    }

    pub fn add_type(&mut self, typ: Type, location: SourceSpan<'filepath>) -> TypeID {
        let id = TypeID(get_id());
        assert!(self.types.insert(id, (typ, location)).is_none());
        id
    }

    pub fn get_location(&self, id: TypeID) -> SourceSpan {
        self.types[&id].1
    }

    pub fn get_type(&self, id: TypeID) -> &Type {
        &self.types[&id].0
    }

    pub fn get_type_mut(&mut self, id: TypeID) -> &mut Type {
        &mut self.types.get_mut(&id).unwrap().0
    }
}

impl<'filepath> Default for Types<'filepath> {
    fn default() -> Self {
        Self::new()
    }
}
