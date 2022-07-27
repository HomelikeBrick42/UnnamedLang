use derive_more::Display;
use enum_as_inner::EnumAsInner;

#[derive(Clone, Debug, Display, EnumAsInner)]
pub enum Type {
    #[display(fmt = "void")]
    Void,
    #[display(fmt = "int")]
    Int,
    #[display(fmt = "bool")]
    Bool,
    #[display(fmt = "proc({}): {}", "parameters_to_string(&parameters)", return_type)]
    Procedure {
        parameters: Vec<Type>,
        return_type: Box<Type>,
    },
}

fn parameters_to_string(parameters: &Vec<Type>) -> String {
    let mut result = String::new();
    for (i, parameter) in parameters.iter().enumerate() {
        if i > 0 {
            result.push_str(", ");
        }
        result.push_str(&parameter.to_string());
    }
    result
}
