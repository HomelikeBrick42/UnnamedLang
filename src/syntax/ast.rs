use super::{GetLocation, SourceSpan, Token};

#[derive(Clone, Debug)]
pub enum Ast<'filepath, 'source> {
    Procedure {
        proc_token: Token<'filepath, 'source>,
        name_token: Option<Token<'filepath, 'source>>,
        open_parenthesis_token: Token<'filepath, 'source>,
        parameters: Vec<AstParameter<'filepath, 'source>>,
        close_parenthesis_token: Token<'filepath, 'source>,
        fat_right_arrow_token: Option<Token<'filepath, 'source>>,
        return_type: Option<Box<Ast<'filepath, 'source>>>,
        body: Option<AstBody<'filepath, 'source>>,
    },
    Return {
        return_token: Token<'filepath, 'source>,
        value: Option<Box<Ast<'filepath, 'source>>>,
    },
    Block {
        open_brace_token: Token<'filepath, 'source>,
        expressions: Vec<Ast<'filepath, 'source>>,
        close_brace_token: Token<'filepath, 'source>,
    },
    Let {
        let_token: Token<'filepath, 'source>,
        name_token: Token<'filepath, 'source>,
        colon_token: Option<Token<'filepath, 'source>>,
        typ: Option<Box<Ast<'filepath, 'source>>>,
        value: Box<Ast<'filepath, 'source>>,
    },
    Var {
        var_token: Token<'filepath, 'source>,
        name_token: Token<'filepath, 'source>,
        colon_token: Token<'filepath, 'source>,
        typ: Box<Ast<'filepath, 'source>>,
        value: Option<Box<Ast<'filepath, 'source>>>,
    },
    Const {
        const_token: Token<'filepath, 'source>,
        name_token: Token<'filepath, 'source>,
        colon_token: Token<'filepath, 'source>,
        typ: Option<Box<Ast<'filepath, 'source>>>,
        value: Box<Ast<'filepath, 'source>>,
    },
    Unary {
        operator: UnaryOperator<'filepath, 'source>,
        operand: Box<Ast<'filepath, 'source>>,
    },
    Binary {
        left: Box<Ast<'filepath, 'source>>,
        operator: BinaryOperator<'filepath, 'source>,
        right: Box<Ast<'filepath, 'source>>,
    },
    Call {
        operand: Box<Ast<'filepath, 'source>>,
        open_parenthesis_token: Token<'filepath, 'source>,
        arguments: Vec<Ast<'filepath, 'source>>,
        close_parenthesis_token: Token<'filepath, 'source>,
    },
    Name {
        name_token: Token<'filepath, 'source>,
    },
    Integer {
        integer_token: Token<'filepath, 'source>,
    },
    String {
        string_token: Token<'filepath, 'source>,
    },
}

impl<'filepath, 'source> GetLocation<'filepath> for Ast<'filepath, 'source> {
    fn get_location(&self) -> SourceSpan<'filepath> {
        match self {
            Ast::Procedure {
                proc_token,
                name_token: _,
                open_parenthesis_token: _,
                parameters: _,
                close_parenthesis_token,
                fat_right_arrow_token: _,
                return_type,
                body,
            } => {
                let last: &dyn GetLocation = if let Some(body) = body {
                    body
                } else if let Some(return_type) = return_type {
                    return_type
                } else {
                    close_parenthesis_token
                };
                SourceSpan::combine(proc_token, last)
            }
            Ast::Return {
                return_token,
                value,
            } => {
                if let Some(value) = value {
                    SourceSpan::combine(return_token, value.get_location())
                } else {
                    return_token.get_location()
                }
            }
            Ast::Block {
                open_brace_token,
                expressions: _,
                close_brace_token,
            } => SourceSpan::combine(open_brace_token, close_brace_token),
            Ast::Let {
                let_token,
                name_token: _,
                colon_token: _,
                typ: _,
                value,
            } => SourceSpan::combine(let_token, value),
            Ast::Var {
                var_token,
                name_token: _,
                colon_token: _,
                typ,
                value,
            } => SourceSpan::combine(var_token, value.as_ref().unwrap_or(typ)),
            Ast::Const {
                const_token,
                name_token: _,
                colon_token: _,
                typ: _,
                value,
            } => SourceSpan::combine(const_token, value),
            Ast::Unary { operator, operand } => SourceSpan::combine(operator, operand),
            Ast::Binary {
                left,
                operator: _,
                right,
            } => SourceSpan::combine(left, right),
            Ast::Call {
                operand,
                open_parenthesis_token: _,
                arguments: _,
                close_parenthesis_token,
            } => SourceSpan::combine(operand, close_parenthesis_token),
            Ast::Name { name_token } => name_token.get_location(),
            Ast::Integer { integer_token } => integer_token.get_location(),
            Ast::String { string_token } => string_token.get_location(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct AstParameter<'filepath, 'source> {
    pub name_token: Token<'filepath, 'source>,
    pub colon_token: Token<'filepath, 'source>,
    pub typ: Ast<'filepath, 'source>,
}

impl<'filepath, 'source> GetLocation<'filepath> for AstParameter<'filepath, 'source> {
    fn get_location(&self) -> SourceSpan<'filepath> {
        SourceSpan::combine(&self.name_token, &self.typ)
    }
}

#[derive(Clone, Debug)]
pub enum AstBody<'filepath, 'source> {
    DoExpression {
        do_token: Token<'filepath, 'source>,
        expression: Box<Ast<'filepath, 'source>>,
    },
    Block(Box<Ast<'filepath, 'source>>),
}

impl<'filepath, 'source> GetLocation<'filepath> for AstBody<'filepath, 'source> {
    fn get_location(&self) -> SourceSpan<'filepath> {
        match self {
            AstBody::DoExpression {
                do_token,
                expression,
            } => SourceSpan::combine(do_token, expression),
            AstBody::Block(block) => block.get_location(),
        }
    }
}

#[derive(Clone, Debug)]
pub enum UnaryOperator<'filepath, 'source> {
    Identity {
        plus_token: Token<'filepath, 'source>,
    },
    Negate {
        minus_token: Token<'filepath, 'source>,
    },
}

impl<'filepath, 'source> GetLocation<'filepath> for UnaryOperator<'filepath, 'source> {
    fn get_location(&self) -> SourceSpan<'filepath> {
        match self {
            UnaryOperator::Identity { plus_token } => plus_token.get_location(),
            UnaryOperator::Negate { minus_token } => minus_token.get_location(),
        }
    }
}

#[derive(Clone, Debug)]
pub enum BinaryOperator<'filepath, 'source> {
    Add {
        plus_token: Token<'filepath, 'source>,
    },
    Subtract {
        minus_token: Token<'filepath, 'source>,
    },
    Multiply {
        asterisk_token: Token<'filepath, 'source>,
    },
    Divide {
        slash_token: Token<'filepath, 'source>,
    },
}

impl<'filepath, 'source> GetLocation<'filepath> for BinaryOperator<'filepath, 'source> {
    fn get_location(&self) -> SourceSpan<'filepath> {
        match self {
            BinaryOperator::Add { plus_token } => plus_token.get_location(),
            BinaryOperator::Subtract { minus_token } => minus_token.get_location(),
            BinaryOperator::Multiply { asterisk_token } => asterisk_token.get_location(),
            BinaryOperator::Divide { slash_token } => slash_token.get_location(),
        }
    }
}
