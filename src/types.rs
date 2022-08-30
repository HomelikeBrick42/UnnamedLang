use std::rc::Rc;

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
