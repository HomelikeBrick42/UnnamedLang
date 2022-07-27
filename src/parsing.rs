use std::rc::Rc;

use derive_more::Display;
use enum_as_inner::EnumAsInner;

use crate::{
    Ast, AstBinary, AstCall, AstFile, AstIf, AstInteger, AstLeftAssign, AstLet, AstName,
    AstProcedure, AstRightAssign, AstScope, AstUnary, AstVar, AstWhile, BinaryOperator, Lexer,
    LexerError, Parameter, ProcedureBody, SourceLocation, SourceSpan, Token, TokenKind,
    UnaryOperator,
};

#[derive(Clone, PartialEq, Debug, Display, EnumAsInner)]
pub enum ParsingError {
    #[display(fmt = "{}", _0)]
    LexerError(LexerError),
    #[display(fmt = "{}: Unexpected token '{}'", "got.location", "got.kind")]
    UnexpectedToken { got: Token },
    #[display(
        fmt = "{}: Expected '{}', but got '{}'",
        "got.location",
        expected,
        "got.kind"
    )]
    ExpectedToken { expected: TokenKind, got: Token },
}

impl From<LexerError> for ParsingError {
    fn from(error: LexerError) -> ParsingError {
        ParsingError::LexerError(error)
    }
}

pub fn parse_file(filepath: String, source: &str) -> Result<Rc<AstFile>, ParsingError> {
    let mut lexer = Lexer::new(filepath.clone(), source);
    let mut statements = vec![];
    'statement_loop: while !lexer.peek_token()?.kind.is_end_of_file() {
        while lexer.peek_token()?.kind == TokenKind::Newline {
            expect_token(&mut lexer, TokenKind::Newline)?;
        }
        if lexer.peek_token()?.kind.is_end_of_file() {
            break 'statement_loop;
        }
        statements.push(parse_statement(&mut lexer)?);
        expect_newline(&mut lexer)?;
    }
    Ok(Rc::new(AstFile {
        location: SourceSpan {
            filepath,
            start: SourceLocation {
                position: 0,
                line: 1,
                column: 1,
            },
            end: lexer.location,
        },
        statements,
    }))
}

pub fn parse_statement(lexer: &mut Lexer) -> Result<Ast, ParsingError> {
    Ok(match lexer.peek_token()?.kind {
        TokenKind::ProcKeyword => {
            let proc_token = expect_token(lexer, TokenKind::ProcKeyword)?;
            let name_token = expect_token(lexer, TokenKind::Name)?;

            expect_token(lexer, TokenKind::OpenParenthesis)?;
            let mut parameters = vec![];
            'parameter_loop: while !lexer.peek_token()?.kind.is_close_parenthesis() {
                if lexer.peek_token()?.kind == TokenKind::Newline {
                    expect_token(lexer, TokenKind::Newline)?;
                }
                if lexer.peek_token()?.kind.is_close_parenthesis() {
                    break 'parameter_loop;
                }

                let name_token = expect_token(lexer, TokenKind::Name)?;
                expect_token(lexer, TokenKind::Colon)?;
                let typ = parse_expression(lexer)?;
                parameters.push(Rc::new(Parameter {
                    location: SourceSpan::combine_spans(&name_token.location, typ.get_location()),
                    name: name_token.data.as_string().unwrap().clone(),
                    typ,
                }));

                if lexer.peek_token()?.kind.is_close_parenthesis() {
                    break 'parameter_loop;
                }
                expect_token(lexer, TokenKind::Comma)?;
            }
            expect_token(lexer, TokenKind::CloseParenthesis)?;

            expect_token(lexer, TokenKind::Colon)?;
            let return_type = parse_expression(lexer)?;

            let body = match lexer.peek_token()?.kind {
                TokenKind::CompilerDirective => ProcedureBody::CompilerGenerated(
                    expect_token(lexer, TokenKind::CompilerDirective)?.location,
                ),
                _ => ProcedureBody::Scope(parse_scope(lexer)?),
            };

            Ast::Procedure(Rc::new(AstProcedure {
                location: SourceSpan::combine_spans(
                    &proc_token.location,
                    match &body {
                        ProcedureBody::CompilerGenerated(location) => location,
                        ProcedureBody::Scope(scope) => &scope.location,
                    },
                ),
                name: name_token.data.as_string().unwrap().clone(),
                parameters,
                return_type,
                body,
            }))
        }

        TokenKind::LetKeyword => {
            let let_token = expect_token(lexer, TokenKind::LetKeyword)?;
            let name_token = expect_token(lexer, TokenKind::Name)?;
            let typ = if lexer.peek_token()?.kind == TokenKind::Colon {
                expect_token(lexer, TokenKind::Colon)?;
                Some(parse_expression(lexer)?)
            } else {
                None
            };
            expect_token(lexer, TokenKind::Equal)?;
            let value = parse_expression(lexer)?;
            Ast::Let(Rc::new(AstLet {
                location: SourceSpan::combine_spans(&let_token.location, value.get_location()),
                name: name_token.data.as_string().unwrap().clone(),
                typ,
                value,
            }))
        }

        TokenKind::VarKeyword => {
            let var_token = expect_token(lexer, TokenKind::VarKeyword)?;
            let name_token = expect_token(lexer, TokenKind::Name)?;
            let typ = if lexer.peek_token()?.kind == TokenKind::Colon {
                expect_token(lexer, TokenKind::Colon)?;
                Some(parse_expression(lexer)?)
            } else {
                None
            };
            expect_token(lexer, TokenKind::LeftArrow)?;
            let value = parse_expression(lexer)?;
            Ast::Var(Rc::new(AstVar {
                location: SourceSpan::combine_spans(&var_token.location, value.get_location()),
                name: name_token.data.as_string().unwrap().clone(),
                typ,
                value,
            }))
        }

        TokenKind::IfKeyword => Ast::If(parse_if(lexer)?),

        TokenKind::WhileKeyword => {
            let while_token = expect_token(lexer, TokenKind::WhileKeyword)?;
            let condition = parse_expression(lexer)?;
            let body = parse_scope(lexer)?;
            Ast::While(Rc::new(AstWhile {
                location: SourceSpan::combine_spans(&while_token.location, &body.location),
                condition,
                body,
            }))
        }

        _ => {
            let expression = parse_expression(lexer)?;
            match lexer.peek_token()?.kind {
                TokenKind::LeftArrow => {
                    expect_token(lexer, TokenKind::LeftArrow)?;
                    let value = parse_expression(lexer)?;
                    Ast::LeftAssign(Rc::new(AstLeftAssign {
                        location: SourceSpan::combine_spans(
                            expression.get_location(),
                            value.get_location(),
                        ),
                        operand: expression,
                        value,
                    }))
                }

                TokenKind::RightArrow => {
                    expect_token(lexer, TokenKind::RightArrow)?;
                    let operand = parse_expression(lexer)?;
                    Ast::RightAssign(Rc::new(AstRightAssign {
                        location: SourceSpan::combine_spans(
                            expression.get_location(),
                            operand.get_location(),
                        ),
                        value: expression,
                        operand,
                    }))
                }

                _ => expression,
            }
        }
    })
}

pub fn parse_scope(lexer: &mut Lexer) -> Result<Rc<AstScope>, ParsingError> {
    let open_brace = expect_token(lexer, TokenKind::OpenBrace)?;
    let mut statements = vec![];
    'statement_loop: while !lexer.peek_token()?.kind.is_close_brace() {
        while lexer.peek_token()?.kind == TokenKind::Newline {
            expect_token(lexer, TokenKind::Newline)?;
        }
        if lexer.peek_token()?.kind.is_close_brace() {
            break 'statement_loop;
        }
        statements.push(parse_statement(lexer)?);
        expect_newline(lexer)?;
    }
    let close_brace = expect_token(lexer, TokenKind::CloseBrace)?;
    Ok(Rc::new(AstScope {
        location: SourceSpan::combine_spans(&open_brace.location, &close_brace.location),
        statements,
    }))
}

pub fn parse_if(lexer: &mut Lexer) -> Result<Rc<AstIf>, ParsingError> {
    let if_token = expect_token(lexer, TokenKind::IfKeyword)?;
    let condition = parse_expression(lexer)?;
    let then_statement = parse_scope(lexer)?;
    let else_statement = if lexer.peek_token()?.kind == TokenKind::ElseKeyword {
        expect_token(lexer, TokenKind::ElseKeyword)?;
        Some(if lexer.peek_token()?.kind.is_if_keyword() {
            Ast::If(parse_if(lexer)?)
        } else {
            Ast::Scope(parse_scope(lexer)?)
        })
    } else {
        None
    };
    Ok(Rc::new(AstIf {
        location: SourceSpan::combine_spans(
            &if_token.location,
            if let Some(ast) = &else_statement {
                ast.get_location()
            } else {
                &then_statement.location
            },
        ),
        condition,
        then_statement: Ast::Scope(then_statement),
        else_statement,
    }))
}

pub fn parse_expression(lexer: &mut Lexer) -> Result<Ast, ParsingError> {
    parse_binary_expression(lexer, 0)
}

fn parse_primary_expression(lexer: &mut Lexer) -> Result<Ast, ParsingError> {
    Ok(match lexer.peek_token()?.kind {
        TokenKind::Name => {
            let name_token = expect_token(lexer, TokenKind::Name)?;
            Ast::Name(Rc::new(AstName {
                location: name_token.location.clone(),
                name: name_token.data.as_string().unwrap().clone(),
            }))
        }

        TokenKind::Integer => {
            let integer_token = expect_token(lexer, TokenKind::Integer)?;
            Ast::Integer(Rc::new(AstInteger {
                location: integer_token.location.clone(),
                integer: integer_token.data.as_integer().unwrap().clone(),
            }))
        }

        _ => {
            return Err(ParsingError::UnexpectedToken {
                got: lexer.next_token()?,
            })
        }
    })
}

fn parse_binary_expression(
    lexer: &mut Lexer,
    parent_precedence: usize,
) -> Result<Ast, ParsingError> {
    fn get_unary_precedence(kind: &TokenKind) -> Option<usize> {
        Some(match kind {
            TokenKind::Plus | TokenKind::Minus => 4,
            _ => return None,
        })
    }

    fn get_binary_precedence(kind: &TokenKind) -> Option<usize> {
        Some(match kind {
            TokenKind::Asterisk | TokenKind::Slash => 3,
            TokenKind::Plus | TokenKind::Minus => 2,
            TokenKind::LessThan
            | TokenKind::GreaterThan
            | TokenKind::LessThanEqual
            | TokenKind::GreaterThanEqual => 1,
            _ => return None,
        })
    }

    let mut left = if let Some(unary_precedence) = get_unary_precedence(&lexer.peek_token()?.kind) {
        let operator_token = lexer.next_token()?;
        let operand = parse_binary_expression(lexer, unary_precedence)?;
        Ast::Unary(Rc::new(AstUnary {
            location: SourceSpan::combine_spans(&operator_token.location, operand.get_location()),
            operator: match operator_token.kind {
                TokenKind::Plus => UnaryOperator::Identity,
                TokenKind::Minus => UnaryOperator::Negation,
                _ => unreachable!(),
            },
            operand,
        }))
    } else {
        parse_primary_expression(lexer)?
    };

    'binary_loop: loop {
        match lexer.peek_token()?.kind {
            TokenKind::OpenParenthesis => {
                expect_token(lexer, TokenKind::OpenParenthesis)?;
                let mut arguments = vec![];
                'parameter_loop: while !lexer.peek_token()?.kind.is_close_parenthesis() {
                    if lexer.peek_token()?.kind == TokenKind::Newline {
                        expect_token(lexer, TokenKind::Newline)?;
                    }
                    if lexer.peek_token()?.kind.is_close_parenthesis() {
                        break 'parameter_loop;
                    }

                    arguments.push(parse_expression(lexer)?);

                    if lexer.peek_token()?.kind.is_close_parenthesis() {
                        break 'parameter_loop;
                    }
                    expect_token(lexer, TokenKind::Comma)?;
                }
                let close_parenthesis = expect_token(lexer, TokenKind::CloseParenthesis)?;

                left = Ast::Call(Rc::new(AstCall {
                    location: SourceSpan::combine_spans(
                        left.get_location(),
                        &close_parenthesis.location,
                    ),
                    operand: left,
                    arguments,
                }));
            }

            _ => {
                let binary_precedence = if let Some(binary_precedence) =
                    get_binary_precedence(&lexer.peek_token()?.kind)
                {
                    if binary_precedence <= parent_precedence {
                        break 'binary_loop;
                    }
                    binary_precedence
                } else {
                    break 'binary_loop;
                };

                let operator_token = lexer.next_token()?;
                let right = parse_binary_expression(lexer, binary_precedence)?;
                left = Ast::Binary(Rc::new(AstBinary {
                    location: SourceSpan::combine_spans(left.get_location(), right.get_location()),
                    left,
                    operator: match operator_token.kind {
                        TokenKind::Plus => BinaryOperator::Add,
                        TokenKind::Minus => BinaryOperator::Subtract,
                        TokenKind::Asterisk => BinaryOperator::Multiply,
                        TokenKind::Slash => BinaryOperator::Divide,
                        TokenKind::LessThan => BinaryOperator::LessThan,
                        TokenKind::GreaterThan => BinaryOperator::GreaterThan,
                        TokenKind::LessThanEqual => BinaryOperator::LessThanEqual,
                        TokenKind::GreaterThanEqual => BinaryOperator::GreaterThanEqual,
                        _ => unreachable!(),
                    },
                    right,
                }))
            }
        }
    }

    Ok(left)
}

fn expect_newline(lexer: &mut Lexer) -> Result<(), ParsingError> {
    let token = lexer.peek_token()?;
    Ok(match token.kind {
        TokenKind::EndOfFile | TokenKind::CloseParenthesis | TokenKind::CloseBrace => {}
        _ => {
            expect_token(lexer, TokenKind::Newline)?;
        }
    })
}

fn expect_token(lexer: &mut Lexer, kind: TokenKind) -> Result<Token, ParsingError> {
    let token = lexer.next_token()?;
    if token.kind != kind {
        Err(ParsingError::ExpectedToken {
            expected: kind,
            got: token,
        })
    } else {
        Ok(token)
    }
}
