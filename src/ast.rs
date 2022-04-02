use std::rc::Rc;

use derive_more::IsVariant;

use crate::token::Token;

#[derive(Debug, Clone, PartialEq, IsVariant)]
pub enum Ast {
    File {
        expressions: Vec<Rc<Ast>>,
        end_of_file_token: Token,
    },
    Name(Token),
    String(Token),
    Integer(Token),
    Float(Token),
    Wildcard(Token),
    Unary {
        operator_token: Token,
        operand: Rc<Ast>,
    },
    Binary {
        left: Rc<Ast>,
        operator_token: Token,
        right: Rc<Ast>,
    },
    ParenthesisedExpression {
        open_parenthesis_token: Token,
        expression: Rc<Ast>,
        close_parenthesis_token: Token,
    },
    Declaration {
        name_or_wildcard_token: Token,
        colon_token: Token,
        typ: Rc<Ast>,
    },
    ConstDeclaration {
        const_token: Token,
        name_or_wildcard_token: Token,
        open_square_bracket_token: Option<Token>,
        generic_parameters: Option<Vec<Parameter>>,
        close_square_bracket_token: Option<Token>,
        colon_token: Option<Token>,
        typ: Option<Rc<Ast>>,
        equals_token: Token,
        value: Rc<Ast>,
    },
    Block {
        open_brace_token: Token,
        expressions: Vec<Rc<Ast>>,
        close_brace_token: Token,
    },
    Function {
        func_token: Token,
        open_parenthesis_token: Token,
        parameters: Vec<Parameter>,
        close_parenthesis_token: Token,
        colon_token: Token,
        typ: Rc<Ast>,
        body: Option<Rc<Ast>>,
    },
    Procedure {
        proc_token: Token,
        open_parenthesis_token: Token,
        parameters: Vec<Parameter>,
        close_parenthesis_token: Token,
        colon_token: Token,
        typ: Rc<Ast>,
        body: Option<Rc<Ast>>,
    },
    Return {
        return_token: Token,
        value: Rc<Ast>,
    },
    If {
        if_token: Token,
        condition: Rc<Ast>,
        then_block: Rc<Ast>,
        else_token: Option<Token>,
        else_block: Option<Rc<Ast>>,
    },
    Call {
        operand: Rc<Ast>,
        open_parenthesis_token: Token,
        arguments: Vec<Rc<Ast>>,
        close_parenthesis_token: Token,
    },
    GenericInstantiation {
        operand: Rc<Ast>,
        open_square_bracket_token: Token,
        arguments: Vec<Rc<Ast>>,
        close_square_bracket_token: Token,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub struct Parameter {
    pub name_or_wildcard_token: Token,
    pub colon_token: Token,
    pub typ: Rc<Ast>,
}
