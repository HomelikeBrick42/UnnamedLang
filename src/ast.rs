use std::rc::Rc;

use enum_as_inner::EnumAsInner;

use crate::SourceSpan;

#[derive(Clone, Debug, EnumAsInner)]
pub enum Ast {
    File(Rc<AstFile>),
    Procedure(Rc<AstProcedure>),
    Scope(Rc<AstScope>),
    If(Rc<AstIf>),
    While(Rc<AstWhile>),
    Call(Rc<AstCall>),
    Unary(Rc<AstUnary>),
    Binary(Rc<AstBinary>),
    Name(Rc<AstName>),
    Integer(Rc<AstInteger>),
}

impl Ast {
    pub fn get_location(&self) -> &SourceSpan {
        match self {
            Ast::File(file) => &file.location,
            Ast::Procedure(procedure) => &procedure.location,
            Ast::Scope(scope) => &scope.location,
            Ast::If(iff) => &iff.location,
            Ast::While(whilee) => &whilee.location,
            Ast::Call(call) => &call.location,
            Ast::Unary(unary) => &unary.location,
            Ast::Binary(binary) => &binary.location,
            Ast::Name(name) => &name.location,
            Ast::Integer(integer) => &integer.location,
        }
    }
}

#[derive(Clone, Debug)]
pub struct AstFile {
    pub location: SourceSpan,
    pub statements: Vec<Ast>,
}

#[derive(Clone, Debug)]
pub struct Parameter {
    pub location: SourceSpan,
    pub name: String,
    pub typ: Ast,
}

#[derive(Clone, Debug, EnumAsInner)]
pub enum ProcedureBody {
    CompilerGenerated,
    Scope(Rc<AstScope>),
}

#[derive(Clone, Debug)]
pub struct AstProcedure {
    pub location: SourceSpan,
    pub name: String,
    pub parameters: Vec<Rc<Parameter>>,
    pub body: ProcedureBody,
}

#[derive(Clone, Debug)]
pub struct AstScope {
    pub location: SourceSpan,
    pub statements: Vec<Ast>,
}

#[derive(Clone, Debug)]
pub struct AstIf {
    pub location: SourceSpan,
    pub condition: Ast,
    pub then_statement: Ast,
    pub else_statement: Option<Ast>,
}

#[derive(Clone, Debug)]
pub struct AstWhile {
    pub location: SourceSpan,
    pub condition: Ast,
    pub looping_statement: Ast,
}

#[derive(Clone, Debug)]
pub struct AstCall {
    pub location: SourceSpan,
    pub operand: Ast,
    pub arguments: Vec<Ast>,
}

#[derive(Clone, Debug, EnumAsInner)]
pub enum UnaryOperator {
    Identity,
    Negation,
}

#[derive(Clone, Debug)]
pub struct AstUnary {
    pub location: SourceSpan,
    pub operator: UnaryOperator,
    pub operand: Ast,
}

#[derive(Clone, Debug, EnumAsInner)]
pub enum BinaryOperator {
    Add,
    Subtract,
    Multiply,
    Divide,
    LessThan,
    GreaterThan,
    LessThanEqual,
    GreaterThanEqual,
}

#[derive(Clone, Debug)]
pub struct AstBinary {
    pub location: SourceSpan,
    pub left: Ast,
    pub operator: BinaryOperator,
    pub right: Ast,
}

#[derive(Clone, Debug)]
pub struct AstName {
    pub location: SourceSpan,
    pub name: String,
}

#[derive(Clone, Debug)]
pub struct AstInteger {
    pub location: SourceSpan,
    pub integer: u128,
}
