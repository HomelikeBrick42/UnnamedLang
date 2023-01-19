use std::collections::HashMap;
use std::fmt::Write;

use derive_more::Display;

use crate::{get_id, syntax::SourceSpan};

#[derive(Clone, Debug, Display, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[display(fmt = "{_0}")]
pub struct TypeID(usize);

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
pub enum Type {
    #[display(
        fmt = "ForwardDeclaration {{ resolved_type = {} }}",
        "_0.map_or(\"unresolved\".to_string(), |id| id.to_string())"
    )]
    ForwardDeclaration(Option<TypeID>),
    #[display(fmt = "void")]
    Void,
    #[display(fmt = "never")]
    Never,
    #[display(fmt = "int")]
    Int,
    #[display(fmt = "char")]
    Char,
    #[display(fmt = "slice[{inner_type}]")]
    Slice { inner_type: TypeID },
    #[display(
        fmt = "Procedure {{ parameter_types = {}, return_type = {return_type} }}",
        "slice_to_string(parameter_types).unwrap()"
    )]
    Procedure {
        parameter_types: Vec<TypeID>,
        return_type: TypeID,
    },
}

impl Type {
    pub fn write(f: &mut dyn std::fmt::Write, typ: TypeID, types: &Types) -> std::fmt::Result {
        match *types.get_type(typ) {
            Type::ForwardDeclaration(typ) => {
                if let Some(typ) = typ {
                    Self::write(f, typ, types)
                } else {
                    write!(f, "_")
                }
            }
            Type::Void => write!(f, "void"),
            Type::Never => write!(f, "never"),
            Type::Int => write!(f, "int"),
            Type::Char => write!(f, "char"),
            Type::Slice { inner_type } => {
                write!(f, "slice[")?;
                Self::write(f, inner_type, types)?;
                write!(f, "]")
            }
            Type::Procedure {
                ref parameter_types,
                return_type,
            } => {
                write!(f, "proc(")?;
                for (i, &parameter_type) in parameter_types.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    Self::write(f, parameter_type, types)?;
                }
                write!(f, ") => ")?;
                Self::write(f, return_type, types)
            }
        }
    }
}

pub struct Types<'filepath> {
    builtin_types: Vec<TypeID>,
    types: HashMap<TypeID, (Type, SourceSpan<'filepath>)>,
}

impl<'filepath> Types<'filepath> {
    pub fn new() -> Self {
        Self {
            builtin_types: Vec::new(),
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

    pub fn get_builtin_type(&mut self, typ: Type) -> TypeID {
        // TODO: find some way to speed this up, though this should already be pretty fast because there shouldnt be that many builtin types
        for &builtin_typ in &self.builtin_types {
            if self.get_type(builtin_typ) == &typ {
                return builtin_typ;
            }
        }
        let new_typ = self.add_type(typ, SourceSpan::builtin_location());
        self.builtin_types.push(new_typ);
        new_typ
    }
}

impl<'filepath> std::fmt::Display for Types<'filepath> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut types = self.types.iter().collect::<Vec<_>>();
        types.sort_by(|&(&a, _), &(&b, _)| a.cmp(&b));
        for (&id, (typ, _)) in types {
            writeln!(f, "typeid = {id}, type = {typ}")?;
        }
        Ok(())
    }
}

impl<'filepath> Default for Types<'filepath> {
    fn default() -> Self {
        Self::new()
    }
}
