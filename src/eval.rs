use std::rc::Rc;

use enum_as_inner::EnumAsInner;

use crate::{
    get_or_add_type_pointer, get_or_add_type_procedure, Ast, AstProcedure, BinaryOperator, Type,
    UnaryOperator,
};

#[derive(Clone, Debug, EnumAsInner)]
pub enum Value {
    Type(Rc<Type>),
    Void,
    Bool(bool),
    S8(i8),
    S16(i16),
    S32(i32),
    S64(i64),
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    Procedure(Rc<AstProcedure>),
    Pointer(Rc<Value>),
}

impl PartialEq for Value {
    fn eq(&self, other: &Value) -> bool {
        match (self, other) {
            (Value::Type(a), Value::Type(b)) => a == b,
            (Value::Void, Value::Void) => true,
            (Value::Bool(a), Value::Bool(b)) => a == b,
            (Value::S8(a), Value::S8(b)) => a == b,
            (Value::S16(a), Value::S16(b)) => a == b,
            (Value::S32(a), Value::S32(b)) => a == b,
            (Value::S64(a), Value::S64(b)) => a == b,
            (Value::U8(a), Value::U8(b)) => a == b,
            (Value::U16(a), Value::U16(b)) => a == b,
            (Value::U32(a), Value::U32(b)) => a == b,
            (Value::U64(a), Value::U64(b)) => a == b,
            (Value::Procedure(a), Value::Procedure(b)) => Rc::as_ptr(a) == Rc::as_ptr(b),
            (Value::Pointer(a), Value::Pointer(b)) => Rc::as_ptr(a) == Rc::as_ptr(b),
            _ => false,
        }
    }
}

pub fn eval(ast: &Ast, type_cache: &mut Vec<Rc<Type>>) -> Rc<Value> {
    match ast {
        Ast::File(file) => {
            for expression in &file.expressions {
                eval(expression, type_cache);
            }
            Value::Void.into()
        }
        Ast::Procedure(procedure) => Value::Procedure(procedure.clone()).into(),
        Ast::ProcedureType(procedure_type) => Value::Type({
            let parameter_types = procedure_type
                .parameter_types
                .iter()
                .map(|typ| eval(typ, type_cache).as_type().unwrap().clone())
                .collect();
            let return_type = eval(&procedure_type.return_type, type_cache)
                .as_type()
                .unwrap()
                .clone();
            get_or_add_type_procedure(
                type_cache,
                parameter_types,
                return_type,
                procedure_type.calling_convention.clone(),
            )
        })
        .into(),
        Ast::Parameter(_) => todo!(),
        Ast::Scope(scope) => {
            for expression in &scope.expressions {
                eval(expression, type_cache);
            }
            Value::Void.into()
        }
        Ast::LetDeclaration(_) => todo!(),
        Ast::VarDeclaration(_) => todo!(),
        Ast::Name(name) => eval(
            name.resolved_declaration.borrow().as_ref().unwrap(),
            type_cache,
        ),
        Ast::Integer(integer) => match integer
            .resolved_type
            .borrow()
            .as_ref()
            .unwrap()
            .as_integer()
            .unwrap()
        {
            (1, true) => Value::S8(integer.value as _).into(),
            (2, true) => Value::S16(integer.value as _).into(),
            (4, true) => Value::S32(integer.value as _).into(),
            (8, true) => Value::S64(integer.value as _).into(),
            (1, false) => Value::U8(integer.value as _).into(),
            (2, false) => Value::U16(integer.value as _).into(),
            (4, false) => Value::U32(integer.value as _).into(),
            (8, false) => Value::U64(integer.value as _).into(),
            _ => unreachable!(),
        },
        Ast::Call(call) => {
            let operand = eval(&call.operand, type_cache);
            _ = operand;
            todo!()
        }
        Ast::Return(_) => todo!(),
        Ast::Unary(unary) => {
            let operand = eval(&unary.operand, type_cache);
            match &unary.operator {
                UnaryOperator::Identity => operand,
                UnaryOperator::Negation => todo!(),
                UnaryOperator::LogicalNot => {
                    Value::Bool(!operand.as_bool().unwrap().clone()).into()
                }
                UnaryOperator::PointerType => Value::Type(get_or_add_type_pointer(
                    type_cache,
                    operand.as_type().unwrap().clone(),
                ))
                .into(),
                UnaryOperator::AddressOf => Value::Pointer(operand).into(),
                UnaryOperator::Dereference => operand.as_pointer().unwrap().clone(),
            }
        }
        Ast::Binary(binary) => {
            let left = eval(&binary.left, type_cache);
            let right = eval(&binary.right, type_cache);
            match &binary.operator {
                BinaryOperator::Add => todo!(),
                BinaryOperator::Subtract => todo!(),
                BinaryOperator::Multiply => todo!(),
                BinaryOperator::Divide => todo!(),
                BinaryOperator::Remainder => todo!(),
                BinaryOperator::Equal => Value::Bool(left == right).into(),
                BinaryOperator::NotEqual => Value::Bool(left != right).into(),
                BinaryOperator::LessThan => todo!(),
                BinaryOperator::GreaterThan => todo!(),
                BinaryOperator::LessThanEqual => todo!(),
                BinaryOperator::GreaterThanEqual => todo!(),
            }
        }
        Ast::If(iff) => {
            let condition = eval(&iff.condition, type_cache).as_bool().unwrap().clone();
            if let Some(else_expression) = &iff.else_expression {
                if condition {
                    eval(&iff.then_expression, type_cache)
                } else {
                    eval(else_expression, type_cache)
                }
            } else {
                if condition {
                    eval(&iff.then_expression, type_cache);
                }
                Value::Void.into()
            }
        }
        Ast::While(whilee) => loop {
            let condition = eval(&whilee.condition, type_cache)
                .as_bool()
                .unwrap()
                .clone();
            if !condition {
                break Value::Void.into();
            }
            eval(&whilee.then_expression, type_cache);
        },
        Ast::Cast(_) => todo!(),
        Ast::Assign(_) => todo!(),
        Ast::Builtin(builtin) => Value::Type(builtin.typ.borrow().as_ref().unwrap().clone()).into(),
    }
}
