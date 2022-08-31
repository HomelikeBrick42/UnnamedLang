use std::rc::Rc;

use derive_more::Display;
use enum_as_inner::EnumAsInner;

use crate::{
    Ast, AstBinary, AstCall, AstFile, AstIf, AstInteger, AstName, AstParameter, AstProcedure,
    AstProcedureBody, AstProcedureType, AstReturn, AstScope, BinaryOperator, Lexer, LexerError,
    Token, TokenKind,
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
    let mut lexer = Lexer::new(filepath, source);
    let mut expressions = vec![];
    loop {
        allow_newlines(&mut lexer)?;
        if lexer.peek_token()?.kind == TokenKind::EndOfFile {
            break;
        }
        expressions.push(parse_expression(&mut lexer)?);
        expect_newline(&mut lexer)?;
    }
    expect_token(&mut lexer, TokenKind::EndOfFile)?;
    Ok(AstFile {
        resolving: false.into(),
        resolved_type: None.into(),
        expressions,
    }
    .into())
}

fn parse_expression(lexer: &mut Lexer) -> Result<Ast, ParsingError> {
    parse_binary_expression(lexer, 0)
}

fn parse_primary_expression(lexer: &mut Lexer) -> Result<Ast, ParsingError> {
    Ok(match lexer.peek_token()?.kind {
        TokenKind::Name => {
            let token = expect_token(lexer, TokenKind::Name)?;
            Ast::Name(
                AstName {
                    resolving: false.into(),
                    resolved_declaration: None.into(),
                    name: token.data.into_string().unwrap(),
                }
                .into(),
            )
        }

        TokenKind::Integer => {
            let token = expect_token(lexer, TokenKind::Integer)?;
            Ast::Integer(
                AstInteger {
                    resolving: false.into(),
                    resolved_type: None.into(),
                    value: token.data.into_integer().unwrap(),
                }
                .into(),
            )
        }

        TokenKind::OpenParenthesis => {
            expect_token(lexer, TokenKind::OpenParenthesis)?;
            let expression = parse_expression(lexer)?;
            expect_token(lexer, TokenKind::CloseParenthesis)?;
            expression
        }

        TokenKind::ProcKeyword => {
            expect_token(lexer, TokenKind::ProcKeyword)?;
            if lexer.peek_token()?.kind == TokenKind::OpenParenthesis {
                expect_token(lexer, TokenKind::OpenParenthesis)?;
                allow_newline(lexer)?;
                let mut parameter_types = vec![];
                while lexer.peek_token()?.kind != TokenKind::CloseParenthesis {
                    let typ = parse_expression(lexer)?;
                    parameter_types.push(typ);
                    expect_comma_and_or_newline(lexer)?;
                }
                expect_token(lexer, TokenKind::CloseParenthesis)?;
                expect_token(lexer, TokenKind::FatRightArrow)?;
                let return_type = parse_expression(lexer)?;
                Ast::ProcedureType(
                    AstProcedureType {
                        resolving: false.into(),
                        resolved_type: None.into(),
                        parameter_types,
                        return_type,
                    }
                    .into(),
                )
            } else {
                let name = expect_token(lexer, TokenKind::Name)?
                    .data
                    .into_string()
                    .unwrap();
                expect_token(lexer, TokenKind::OpenParenthesis)?;
                allow_newline(lexer)?;
                let mut parameters = vec![];
                while lexer.peek_token()?.kind != TokenKind::CloseParenthesis {
                    let name = expect_token(lexer, TokenKind::Name)?
                        .data
                        .into_string()
                        .unwrap();
                    expect_token(lexer, TokenKind::Colon)?;
                    let typ = parse_expression(lexer)?;
                    parameters.push(
                        AstParameter {
                            resolving: false.into(),
                            resolved_type: None.into(),
                            name,
                            typ,
                        }
                        .into(),
                    );
                    expect_comma_and_or_newline(lexer)?;
                }
                expect_token(lexer, TokenKind::CloseParenthesis)?;
                expect_token(lexer, TokenKind::FatRightArrow)?;
                let return_type = parse_expression(lexer)?;
                let body = if lexer.peek_token()?.kind == TokenKind::ExternDirective {
                    expect_token(lexer, TokenKind::ExternDirective)?;
                    let extern_name = expect_token(lexer, TokenKind::String)?
                        .data
                        .into_string()
                        .unwrap();
                    AstProcedureBody::ExternName(extern_name)
                } else {
                    AstProcedureBody::Scope(parse_scope(lexer)?)
                };
                Ast::Procedure(
                    AstProcedure {
                        resolving: false.into(),
                        resolved_type: None.into(),
                        name,
                        parameters,
                        return_type,
                        body,
                    }
                    .into(),
                )
            }
        }

        TokenKind::OpenBrace => Ast::Scope(parse_scope(lexer)?),

        TokenKind::ReturnKeyword => {
            expect_token(lexer, TokenKind::ReturnKeyword)?;
            let value = if !matches!(
                lexer.peek_token()?.kind,
                TokenKind::Newline
                    | TokenKind::EndOfFile
                    | TokenKind::CloseBrace
                    | TokenKind::CloseParenthesis
            ) {
                Some(parse_expression(lexer)?)
            } else {
                None
            };
            Ast::Return(
                AstReturn {
                    resolving: false.into(),
                    resolved_type: None.into(),
                    value,
                }
                .into(),
            )
        }

        TokenKind::IfKeyword => {
            expect_token(lexer, TokenKind::IfKeyword)?;
            let condition = parse_expression(lexer)?;
            let then_expression = parse_expression(lexer)?;
            let else_expression = if lexer.peek_token()?.kind == TokenKind::ElseKeyword {
                expect_token(lexer, TokenKind::ElseKeyword)?;
                Some(parse_expression(lexer)?)
            } else {
                None
            };
            Ast::If(
                AstIf {
                    resolving: false.into(),
                    resolved_type: None.into(),
                    condition,
                    then_expression,
                    else_expression,
                }
                .into(),
            )
        }

        _ => {
            let token = lexer.next_token()?;
            return Err(ParsingError::UnexpectedToken { got: token });
        }
    })
}

fn parse_scope(lexer: &mut Lexer) -> Result<Rc<AstScope>, ParsingError> {
    expect_token(lexer, TokenKind::OpenBrace)?;
    let mut expressions = vec![];
    loop {
        allow_newlines(lexer)?;
        if lexer.peek_token()?.kind == TokenKind::CloseBrace {
            break;
        }
        expressions.push(parse_expression(lexer)?);
        expect_newline(lexer)?;
    }
    expect_token(lexer, TokenKind::CloseBrace)?;
    Ok(AstScope {
        resolving: false.into(),
        resolved_type: None.into(),
        expressions,
    }
    .into())
}

fn parse_binary_expression(
    lexer: &mut Lexer,
    parent_precedence: usize,
) -> Result<Ast, ParsingError> {
    fn get_unary_precedence(_kind: TokenKind) -> usize {
        0
    }

    fn get_binary_precedence(kind: TokenKind) -> usize {
        match kind {
            TokenKind::Asterisk | TokenKind::Slash => 3,
            TokenKind::Plus | TokenKind::Minus => 2,
            TokenKind::EqualEqual
            | TokenKind::ExclamationMarkEqual
            | TokenKind::LessThan
            | TokenKind::GreaterThan
            | TokenKind::LessThanEqual
            | TokenKind::GreaterThanEqual => 1,
            _ => 0,
        }
    }

    let mut left = {
        let unary_precedence = get_unary_precedence(lexer.peek_token()?.kind);
        if unary_precedence > 0 {
            todo!()
        } else {
            parse_primary_expression(lexer)?
        }
    };

    loop {
        left = match lexer.peek_token()?.kind {
            TokenKind::OpenParenthesis => {
                expect_token(lexer, TokenKind::OpenParenthesis)?;
                allow_newline(lexer)?;
                let mut arguments = vec![];
                while lexer.peek_token()?.kind != TokenKind::CloseParenthesis {
                    arguments.push(parse_expression(lexer)?);
                    expect_comma_and_or_newline(lexer)?;
                }
                expect_token(lexer, TokenKind::CloseParenthesis)?;
                Ast::Call(
                    AstCall {
                        resolving: false.into(),
                        resolved_type: None.into(),
                        operand: left,
                        arguments,
                    }
                    .into(),
                )
            }

            kind => {
                let binary_precedence = get_binary_precedence(kind);
                if binary_precedence <= parent_precedence {
                    break;
                }
                let operator = match lexer.next_token()?.kind {
                    TokenKind::Plus => BinaryOperator::Add,
                    TokenKind::Minus => BinaryOperator::Subtract,
                    TokenKind::Asterisk => BinaryOperator::Multiply,
                    TokenKind::Slash => BinaryOperator::Divide,
                    TokenKind::EqualEqual => BinaryOperator::Equal,
                    TokenKind::ExclamationMarkEqual => BinaryOperator::NotEqual,
                    TokenKind::LessThan => BinaryOperator::LessThan,
                    TokenKind::GreaterThan => BinaryOperator::GreaterThan,
                    TokenKind::LessThanEqual => BinaryOperator::LessThanEqual,
                    TokenKind::GreaterThanEqual => BinaryOperator::GreaterThanEqual,
                    _ => unreachable!(),
                };
                let right = parse_binary_expression(lexer, binary_precedence)?;
                Ast::Binary(
                    AstBinary {
                        resolving: false.into(),
                        resolved_type: None.into(),
                        left,
                        operator,
                        right,
                    }
                    .into(),
                )
            }
        };
    }

    Ok(left)
}

fn allow_newlines(lexer: &mut Lexer) -> Result<(), ParsingError> {
    while lexer.peek_token()?.kind == TokenKind::Newline {
        lexer.next_token()?;
    }
    Ok(())
}

fn allow_newline(lexer: &mut Lexer) -> Result<(), ParsingError> {
    if lexer.peek_token()?.kind == TokenKind::Newline {
        lexer.next_token()?;
    }
    Ok(())
}

fn expect_comma_and_or_newline(lexer: &mut Lexer) -> Result<(), ParsingError> {
    let token = lexer.peek_token()?;
    match token.kind {
        TokenKind::CloseParenthesis | TokenKind::CloseBrace => {}
        _ => {
            expect_token(lexer, TokenKind::Comma)?;
            allow_newline(lexer)?;
        }
    }
    Ok(())
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
