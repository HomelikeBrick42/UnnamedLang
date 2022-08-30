use std::{
    cell::{Cell, RefCell},
    ffi::c_void,
    rc::Rc,
};

use derive_more::IsVariant;
use enum_as_inner::EnumAsInner;

use crate::Type;

#[derive(Clone, Debug, PartialEq, IsVariant, EnumAsInner)]
pub enum Ast {
    File(Rc<AstFile>),
    Procedure(Rc<AstProcedure>),
    ProcedureType(Rc<AstProcedureType>),
    Parameter(Rc<AstParameter>),
    Scope(Rc<AstScope>),
    VarDeclaration(Rc<AstVar>),
    Name(Rc<AstName>),
    Integer(Rc<AstInteger>),
    Call(Rc<AstCall>),
    Return(Rc<AstReturn>),
    Builtin(Rc<AstBuiltin>),
}

impl Ast {
    pub fn get_type(&self) -> Option<Rc<Type>> {
        match self {
            Ast::File(file) => file.resolved_type.borrow().clone(),
            Ast::Procedure(procedure) => procedure.resolved_type.borrow().clone(),
            Ast::ProcedureType(procedure_type) => procedure_type.resolved_type.borrow().clone(),
            Ast::Parameter(parameter) => parameter.resolved_type.borrow().clone(),
            Ast::Scope(scope) => scope.resolved_type.borrow().clone(),
            Ast::VarDeclaration(declaration) => declaration.resolved_type.borrow().clone(),
            Ast::Name(name) => name
                .resolved_declaration
                .borrow()
                .as_ref()
                .map(Ast::get_type)
                .flatten(),
            Ast::Integer(integer) => integer.resolved_type.borrow().clone(),
            Ast::Call(call) => call.resolved_type.borrow().clone(),
            Ast::Return(returnn) => returnn.resolved_type.borrow().clone(),
            Ast::Builtin(builtin) => match builtin.as_ref() {
                AstBuiltin::Type => Some(Type::Type.into()),
                AstBuiltin::Void => Some(Type::Type.into()),
                AstBuiltin::IntegerType { size: _, signed: _ } => Some(Type::Type.into()),
            },
        }
    }

    pub fn set_resolving(&self, value: bool) {
        match self {
            Ast::File(file) => file.resolving.set(value),
            Ast::Procedure(procedure) => procedure.resolving.set(value),
            Ast::ProcedureType(procedure_type) => procedure_type.resolving.set(value),
            Ast::Parameter(parameter) => parameter.resolving.set(value),
            Ast::Scope(scope) => scope.resolving.set(value),
            Ast::VarDeclaration(declaration) => declaration.resolving.set(value),
            Ast::Name(name) => name.resolving.set(value),
            Ast::Integer(integer) => integer.resolving.set(value),
            Ast::Call(call) => call.resolving.set(value),
            Ast::Return(returnn) => returnn.resolving.set(value),
            Ast::Builtin(builtin) => match builtin.as_ref() {
                AstBuiltin::Type => (),
                AstBuiltin::Void => (),
                AstBuiltin::IntegerType { size: _, signed: _ } => (),
            },
        }
    }

    pub fn get_resolving(&self) -> bool {
        match self {
            Ast::File(file) => file.resolving.get(),
            Ast::Procedure(procedure) => procedure.resolving.get(),
            Ast::ProcedureType(procedure_type) => procedure_type.resolving.get(),
            Ast::Parameter(parameter) => parameter.resolving.get(),
            Ast::Scope(scope) => scope.resolving.get(),
            Ast::VarDeclaration(declaration) => declaration.resolving.get(),
            Ast::Name(name) => name.resolving.get(),
            Ast::Integer(integer) => integer.resolving.get(),
            Ast::Call(call) => call.resolving.get(),
            Ast::Return(returnn) => returnn.resolving.get(),
            Ast::Builtin(builtin) => match builtin.as_ref() {
                AstBuiltin::Type => false,
                AstBuiltin::Void => false,
                AstBuiltin::IntegerType { size: _, signed: _ } => false,
            },
        }
    }

    pub(crate) fn get_ptr(&self) -> *const c_void {
        match self {
            Ast::File(file) => Rc::as_ptr(file) as *const _,
            Ast::Procedure(procedure) => Rc::as_ptr(procedure) as *const _,
            Ast::ProcedureType(procedure_type) => Rc::as_ptr(procedure_type) as *const _,
            Ast::Parameter(parameter) => Rc::as_ptr(parameter) as *const _,
            Ast::Scope(scope) => Rc::as_ptr(scope) as *const _,
            Ast::VarDeclaration(declaration) => Rc::as_ptr(declaration) as *const _,
            Ast::Name(name) => Rc::as_ptr(name) as *const _,
            Ast::Integer(integer) => Rc::as_ptr(integer) as *const _,
            Ast::Call(call) => Rc::as_ptr(call) as *const _,
            Ast::Return(returnn) => Rc::as_ptr(returnn) as *const _,
            Ast::Builtin(builtin) => Rc::as_ptr(builtin) as *const _,
        }
    }
}

type ResolvedType = RefCell<Option<Rc<Type>>>;

#[derive(Clone, Debug, PartialEq)]
pub struct AstFile {
    pub resolving: Cell<bool>,
    pub resolved_type: ResolvedType,
    pub expressions: Vec<Ast>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct AstProcedure {
    pub resolving: Cell<bool>,
    pub resolved_type: ResolvedType,
    pub name: String,
    pub parameters: Vec<Rc<AstParameter>>,
    pub return_type: Ast,
    pub body: AstProcedureBody,
}

#[derive(Clone, Debug, PartialEq)]
pub struct AstProcedureType {
    pub resolving: Cell<bool>,
    pub resolved_type: ResolvedType,
    pub parameter_types: Vec<Ast>,
    pub return_type: Ast,
}

#[derive(Clone, Debug, PartialEq)]
pub struct AstParameter {
    pub resolving: Cell<bool>,
    pub resolved_type: ResolvedType,
    pub name: String,
    pub typ: Ast,
}

#[derive(Clone, Debug, PartialEq, IsVariant, EnumAsInner)]
pub enum AstProcedureBody {
    ExternName(String),
    Scope(Rc<AstScope>),
}

#[derive(Clone, Debug, PartialEq)]
pub struct AstScope {
    pub resolving: Cell<bool>,
    pub resolved_type: ResolvedType,
    pub expressions: Vec<Ast>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct AstVar {
    pub resolving: Cell<bool>,
    pub resolved_type: ResolvedType,
    pub name: String,
    pub typ: Ast,
    pub value: Ast,
}

#[derive(Clone, Debug, PartialEq)]
pub struct AstName {
    pub resolving: Cell<bool>,
    pub name: String,
    pub resolved_declaration: RefCell<Option<Ast>>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct AstInteger {
    pub resolving: Cell<bool>,
    pub resolved_type: ResolvedType,
    pub value: u128,
}

#[derive(Clone, Debug, PartialEq)]
pub struct AstCall {
    pub resolving: Cell<bool>,
    pub resolved_type: ResolvedType,
    pub operand: Ast,
    pub arguments: Vec<Ast>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct AstReturn {
    pub resolving: Cell<bool>,
    pub resolved_type: ResolvedType,
    pub value: Option<Ast>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum AstBuiltin {
    Type,
    Void,
    IntegerType { size: usize, signed: bool },
}
