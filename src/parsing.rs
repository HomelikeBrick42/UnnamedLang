use std::rc::Rc;

use derive_more::Display;
use enum_as_inner::EnumAsInner;

use crate::{
    Ast, AstBinary, AstFile, AstUnary, BinaryOperator, Lexer, LexerError, SourceLocation,
    SourceSpan, Token, TokenKind, UnaryOperator,
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
    match lexer.peek_token()?.kind {
        TokenKind::ProcKeyword => todo!(),
        TokenKind::LetKeyword => todo!(),
        TokenKind::VarKeyword => todo!(),
        TokenKind::IfKeyword => todo!(),
        TokenKind::WhileKeyword => todo!(),
        _ => parse_expression(lexer),
    }
}

pub fn parse_expression(lexer: &mut Lexer) -> Result<Ast, ParsingError> {
    parse_binary_expression(lexer, 0)
}

fn parse_primary_expression(lexer: &mut Lexer) -> Result<Ast, ParsingError> {
    match lexer.peek_token()?.kind {
        _ => Err(ParsingError::UnexpectedToken {
            got: lexer.next_token()?,
        }),
    }
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
            TokenKind::OpenParenthesis => todo!(),

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
