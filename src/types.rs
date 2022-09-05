use std::{fmt::Display, rc::Rc};

use enum_as_inner::EnumAsInner;

use crate::CallingConvention;

#[derive(Clone, Debug, EnumAsInner)]
pub enum Type {
    Type,
    Void,
    Bool,
    Integer {
        size: usize,
        signed: bool,
    },
    Procedure {
        parameter_types: Vec<Rc<Type>>,
        return_type: Rc<Type>,
        calling_convention: CallingConvention,
    },
    Pointer {
        pointed_to: Rc<Type>,
    },
}

impl PartialEq for Type {
    fn eq(&self, other: &Self) -> bool {
        self as *const _ == other as *const _
    }
}

impl Eq for Type {}

impl Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Type::Type => write!(f, "type"),
            Type::Void => write!(f, "void"),
            Type::Bool => write!(f, "bool"),
            Type::Integer { size, signed } => {
                write!(f, "{}{}", if *signed { "s" } else { "u" }, size * 8)
            }
            Type::Procedure {
                parameter_types,
                return_type,
                calling_convention,
            } => {
                write!(f, "proc(")?;
                for (i, parameter_type) in parameter_types.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{parameter_type}")?;
                }
                write!(f, "): ")?;
                write!(f, "{return_type} {calling_convention}")
            }
            Type::Pointer { pointed_to } => write!(f, "^{pointed_to}"),
        }
    }
}

pub fn get_or_add_type_type(type_cache: &mut Vec<Rc<Type>>) -> Rc<Type> {
    for typ in type_cache.iter() {
        if typ.is_type() {
            return typ.clone();
        }
    }
    let typ: Rc<_> = Type::Type.into();
    type_cache.push(typ.clone());
    typ
}

pub fn get_or_add_type_void(type_cache: &mut Vec<Rc<Type>>) -> Rc<Type> {
    for typ in type_cache.iter() {
        if typ.is_void() {
            return typ.clone();
        }
    }
    let typ: Rc<_> = Type::Void.into();
    type_cache.push(typ.clone());
    typ
}

pub fn get_or_add_type_bool(type_cache: &mut Vec<Rc<Type>>) -> Rc<Type> {
    for typ in type_cache.iter() {
        if typ.is_bool() {
            return typ.clone();
        }
    }
    let typ: Rc<_> = Type::Bool.into();
    type_cache.push(typ.clone());
    typ
}

pub fn get_or_add_type_integer(
    type_cache: &mut Vec<Rc<Type>>,
    size: usize,
    signed: bool,
) -> Rc<Type> {
    for typ in type_cache.iter() {
        if let Some((&typ_size, &typ_signed)) = typ.as_integer() {
            if typ_size == size && typ_signed == signed {
                return typ.clone();
            }
        }
    }
    let typ: Rc<_> = Type::Integer { size, signed }.into();
    type_cache.push(typ.clone());
    typ
}

pub fn get_or_add_type_procedure(
    type_cache: &mut Vec<Rc<Type>>,
    parameter_types: Vec<Rc<Type>>,
    return_type: Rc<Type>,
    calling_convention: CallingConvention,
) -> Rc<Type> {
    for typ in type_cache.iter() {
        if let Some((typ_parameter_types, typ_return_type, typ_calling_convention)) =
            typ.as_procedure()
        {
            if typ_parameter_types == &parameter_types
                && typ_return_type == &return_type
                && typ_calling_convention == &calling_convention
            {
                return typ.clone();
            }
        }
    }
    let typ: Rc<_> = Type::Procedure {
        parameter_types,
        return_type,
        calling_convention,
    }
    .into();
    type_cache.push(typ.clone());
    typ
}

pub fn get_or_add_type_pointer(type_cache: &mut Vec<Rc<Type>>, pointed_to: Rc<Type>) -> Rc<Type> {
    for typ in type_cache.iter() {
        if let Some(typ_pointed_to) = typ.as_pointer() {
            if typ_pointed_to == &pointed_to {
                return typ.clone();
            }
        }
    }
    let typ: Rc<_> = Type::Pointer { pointed_to }.into();
    type_cache.push(typ.clone());
    typ
}
