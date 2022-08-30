use std::{fmt::Display, rc::Rc};

use enum_as_inner::EnumAsInner;

#[derive(Clone, Debug, PartialEq, EnumAsInner)]
pub enum Type {
    Type,
    Void,
    S64,
    Procedure {
        parameter_types: Vec<Rc<Type>>,
        return_type: Rc<Type>,
    },
    Pointer {
        pointed_to: Rc<Type>,
    },
}

impl Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Type::Type => write!(f, "type"),
            Type::Void => write!(f, "void"),
            Type::S64 => write!(f, "s64"),
            Type::Procedure {
                parameter_types,
                return_type,
            } => {
                write!(f, "proc(")?;
                for (i, parameter_type) in parameter_types.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{parameter_type}")?;
                }
                write!(f, "): ")?;
                write!(f, "{return_type}")
            }
            Type::Pointer { pointed_to } => write!(f, "^{pointed_to}"),
        }
    }
}
