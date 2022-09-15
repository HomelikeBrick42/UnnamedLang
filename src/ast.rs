use std::rc::Rc;

use derive_more::{Display, IsVariant};
use enum_as_inner::EnumAsInner;

use crate::SourceSpan;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, IsVariant, EnumAsInner)]
pub enum Ast {
    File(Rc<AstFile>),
    Procedure(Rc<AstProcedure>),
    ProcedureType(Rc<AstProcedureType>),
    Parameter(Rc<AstParameter>),
    Scope(Rc<AstScope>),
    LetDeclaration(Rc<AstLet>),
    VarDeclaration(Rc<AstVar>),
    Name(Rc<AstName>),
    Integer(Rc<AstInteger>),
    Call(Rc<AstCall>),
    Return(Rc<AstReturn>),
    Unary(Rc<AstUnary>),
    Binary(Rc<AstBinary>),
    If(Rc<AstIf>),
    While(Rc<AstWhile>),
    Cast(Rc<AstCast>),
    Assign(Rc<AstAssign>),
}

impl Ast {
    pub fn get_location(&self) -> SourceSpan {
        match self {
            Ast::File(ast) => ast.location.clone(),
            Ast::Procedure(ast) => ast.location.clone(),
            Ast::ProcedureType(ast) => ast.location.clone(),
            Ast::Parameter(ast) => ast.location.clone(),
            Ast::Scope(ast) => ast.location.clone(),
            Ast::LetDeclaration(ast) => ast.location.clone(),
            Ast::VarDeclaration(ast) => ast.location.clone(),
            Ast::Name(ast) => ast.location.clone(),
            Ast::Integer(ast) => ast.location.clone(),
            Ast::Call(ast) => ast.location.clone(),
            Ast::Return(ast) => ast.location.clone(),
            Ast::Unary(ast) => ast.location.clone(),
            Ast::Binary(ast) => ast.location.clone(),
            Ast::If(ast) => ast.location.clone(),
            Ast::While(ast) => ast.location.clone(),
            Ast::Cast(ast) => ast.location.clone(),
            Ast::Assign(ast) => ast.location.clone(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct AstFile {
    pub location: SourceSpan,
    pub expressions: Vec<Ast>,
}

#[derive(Clone, Debug, Display, PartialEq, Eq, PartialOrd, Ord, EnumAsInner)]
pub enum CallingConvention {
    #[display(fmt = "#cdecl")]
    CDecl,
    #[display(fmt = "#stdcall")]
    StdCall,
    #[display(fmt = "#fastcall")]
    FastCall,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct AstProcedure {
    pub location: SourceSpan,
    pub name: String,
    pub parameters: Vec<Rc<AstParameter>>,
    pub return_type: Ast,
    pub calling_convention: CallingConvention,
    pub body: AstProcedureBody,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct AstProcedureType {
    pub location: SourceSpan,
    pub parameter_types: Vec<Ast>,
    pub calling_convention: CallingConvention,
    pub return_type: Ast,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct AstParameter {
    pub location: SourceSpan,
    pub mutable: bool,
    pub name: String,
    pub typ: Ast,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, IsVariant, EnumAsInner)]
pub enum AstProcedureBody {
    ExternName(String),
    Scope(Rc<AstScope>),
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct AstScope {
    pub location: SourceSpan,
    pub expressions: Vec<Ast>,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct AstLet {
    pub location: SourceSpan,
    pub name: String,
    pub typ: Option<Ast>,
    pub value: Ast,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct AstVar {
    pub location: SourceSpan,
    pub name: String,
    pub typ: Option<Ast>,
    pub value: Ast,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct AstName {
    pub location: SourceSpan,
    pub name: String,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct AstInteger {
    pub location: SourceSpan,
    pub value: u128,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct AstCall {
    pub location: SourceSpan,
    pub operand: Ast,
    pub arguments: Vec<Ast>,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct AstReturn {
    pub location: SourceSpan,
    pub value: Option<Ast>,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, EnumAsInner)]
pub enum UnaryOperator {
    Identity,
    Negation,
    LogicalNot,
    PointerType,
    AddressOf,
    Dereference,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct AstUnary {
    pub location: SourceSpan,
    pub operator: UnaryOperator,
    pub operand: Ast,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, EnumAsInner)]
pub enum BinaryOperator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Remainder,
    Equal,
    NotEqual,
    LessThan,
    GreaterThan,
    LessThanEqual,
    GreaterThanEqual,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct AstBinary {
    pub location: SourceSpan,
    pub left: Ast,
    pub operator: BinaryOperator,
    pub right: Ast,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct AstIf {
    pub location: SourceSpan,
    pub condition: Ast,
    pub then_expression: Ast,
    pub else_expression: Option<Ast>,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct AstWhile {
    pub location: SourceSpan,
    pub condition: Ast,
    pub then_expression: Ast,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct AstCast {
    pub location: SourceSpan,
    pub typ: Ast,
    pub operand: Ast,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, EnumAsInner)]
pub enum AstAssignDirection {
    Left,
    Right,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct AstAssign {
    pub location: SourceSpan,
    pub direction: AstAssignDirection,
    pub operand: Ast,
    pub value: Ast,
}
