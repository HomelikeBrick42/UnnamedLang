use std::rc::Rc;

use enum_as_inner::EnumAsInner;

use crate::SourceSpan;

#[derive(Clone, Debug, PartialEq, EnumAsInner)]
pub enum Ast {
    File(Rc<AstFile>),
    Procedure(Rc<AstProcedure>),
    Return(Rc<AstReturn>),
    Scope(Rc<AstScope>),
    Let(Rc<AstLet>),
    Var(Rc<AstVar>),
    LeftAssign(Rc<AstLeftAssign>),
    RightAssign(Rc<AstRightAssign>),
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
            Ast::Return(returnn) => &returnn.location,
            Ast::Scope(scope) => &scope.location,
            Ast::Let(lett) => &lett.location,
            Ast::Var(var) => &var.location,
            Ast::LeftAssign(left_assign) => &left_assign.location,
            Ast::RightAssign(right_assign) => &right_assign.location,
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

#[derive(Clone, Debug, PartialEq)]
pub struct AstFile {
    pub location: SourceSpan,
    pub statements: Vec<Ast>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Parameter {
    pub location: SourceSpan,
    pub name: String,
    pub typ: Ast,
}

#[derive(Clone, Debug, PartialEq, EnumAsInner)]
pub enum ProcedureBody {
    CompilerGenerated(SourceSpan),
    Scope(Ast),
}

#[derive(Clone, Debug, PartialEq)]
pub struct AstProcedure {
    pub location: SourceSpan,
    pub name: String,
    pub parameters: Vec<Rc<Parameter>>,
    pub return_type: Ast,
    pub body: ProcedureBody,
}

#[derive(Clone, Debug, PartialEq)]
pub struct AstReturn {
    pub location: SourceSpan,
    pub value: Option<Ast>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct AstScope {
    pub location: SourceSpan,
    pub statements: Vec<Ast>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct AstLet {
    pub location: SourceSpan,
    pub name: String,
    pub typ: Option<Ast>,
    pub value: Ast,
}

#[derive(Clone, Debug, PartialEq)]
pub struct AstVar {
    pub location: SourceSpan,
    pub name: String,
    pub typ: Option<Ast>,
    pub value: Ast,
}

#[derive(Clone, Debug, PartialEq)]
pub struct AstLeftAssign {
    pub location: SourceSpan,
    pub operand: Ast,
    pub value: Ast,
}

#[derive(Clone, Debug, PartialEq)]
pub struct AstRightAssign {
    pub location: SourceSpan,
    pub value: Ast,
    pub operand: Ast,
}

#[derive(Clone, Debug, PartialEq)]
pub struct AstIf {
    pub location: SourceSpan,
    pub condition: Ast,
    pub then_statement: Ast,
    pub else_statement: Option<Ast>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct AstWhile {
    pub location: SourceSpan,
    pub condition: Ast,
    pub body: Ast,
}

#[derive(Clone, Debug, PartialEq)]
pub struct AstCall {
    pub location: SourceSpan,
    pub operand: Ast,
    pub arguments: Vec<Ast>,
}

#[derive(Clone, Debug, PartialEq, EnumAsInner)]
pub enum UnaryOperator {
    Identity,
    Negation,
}

#[derive(Clone, Debug, PartialEq)]
pub struct AstUnary {
    pub location: SourceSpan,
    pub operator: UnaryOperator,
    pub operand: Ast,
}

#[derive(Clone, Debug, PartialEq, EnumAsInner)]
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

#[derive(Clone, Debug, PartialEq)]
pub struct AstBinary {
    pub location: SourceSpan,
    pub left: Ast,
    pub operator: BinaryOperator,
    pub right: Ast,
}

#[derive(Clone, Debug, PartialEq)]
pub struct AstName {
    pub location: SourceSpan,
    pub name: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct AstInteger {
    pub location: SourceSpan,
    pub integer: u128,
}
