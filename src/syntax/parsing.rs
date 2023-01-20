use derive_more::Display;

use super::{
    Ast, AstBody, AstParameter, BinaryOperator, GetLocation, Lexer, LexerError, SourceSpan, Token,
    TokenKind, UnaryOperator,
};

#[derive(Debug, Display)]
pub enum ParsingError<'filepath, 'source> {
    #[display(fmt = "{_0}")]
    LexerError(LexerError<'filepath>),
    #[display(fmt = "{}: Unexpected {token} token", "token.get_location()")]
    UnexpectedToken { token: Token<'filepath, 'source> },
    #[display(fmt = "{}: Unexpected name token, but got {got}", "got.get_location()")]
    ExpectedNameToken { got: Token<'filepath, 'source> },
    #[display(fmt = "{}: Expected {expected} but got {got}", "got.get_location()")]
    ExpectedTokenButGot {
        expected: TokenKind<'source>,
        got: Token<'filepath, 'source>,
    },
    #[display(fmt = "Unexpected end of source file '{filepath}'")]
    UnexpectedEndOfFile { filepath: &'filepath str },
    #[display(fmt = "{location}: Procedure with name must have a body")]
    ProcedureWithNameMustHaveBody { location: SourceSpan<'filepath> },
}

impl<'filepath, 'source> From<LexerError<'filepath>> for ParsingError<'filepath, 'source> {
    fn from(error: LexerError<'filepath>) -> Self {
        ParsingError::LexerError(error)
    }
}

pub fn parse_file<'filepath, 'source>(
    filepath: &'filepath str,
    source: &'source str,
) -> Result<Vec<Ast<'filepath, 'source>>, ParsingError<'filepath, 'source>> {
    let mut lexer = Lexer::new(filepath, source);
    let mut expressions = vec![];
    while lexer.peek_token()?.is_some() {
        while let Some(Token {
            kind: TokenKind::Newline,
            ..
        }) = lexer.peek_token()?
        {
            next_token(&mut lexer)?;
        }
        expressions.push(parse_expression(&mut lexer)?);
        expect_token(&mut lexer, TokenKind::Newline)?;
    }
    Ok(expressions)
}

fn parse_expression<'filepath, 'source>(
    lexer: &mut Lexer<'filepath, 'source>,
) -> Result<Ast<'filepath, 'source>, ParsingError<'filepath, 'source>> {
    parse_binary_expression(lexer, 0)
}

fn parse_primary_expression<'filepath, 'source>(
    lexer: &mut Lexer<'filepath, 'source>,
) -> Result<Ast<'filepath, 'source>, ParsingError<'filepath, 'source>> {
    let mut expression = match next_token(lexer)? {
        name_token @ Token {
            kind: TokenKind::Name(_),
            ..
        } => Ast::Name { name_token },
        integer_token @ Token {
            kind: TokenKind::Integer(_),
            ..
        } => Ast::Integer { integer_token },
        string_token @ Token {
            kind: TokenKind::String(_),
            ..
        } => Ast::String { string_token },
        let_token @ Token {
            kind: TokenKind::LetKeyword,
            ..
        } => {
            _ = let_token;
            todo!();
        }
        var_token @ Token {
            kind: TokenKind::VarKeyword,
            ..
        } => {
            _ = var_token;
            todo!();
        }
        const_token @ Token {
            kind: TokenKind::ConstKeyword,
            ..
        } => {
            _ = const_token;
            todo!();
        }
        proc_token @ Token {
            kind: TokenKind::ProcKeyword,
            ..
        } => {
            let name_token = if let Some(Token {
                kind: TokenKind::Name(_),
                ..
            }) = lexer.peek_token()?
            {
                Some(next_token(lexer)?)
            } else {
                None
            };
            let open_parenthesis_token = expect_token(lexer, TokenKind::OpenParenthesis)?;
            let mut parameters = vec![];
            while !matches!(
                lexer.peek_token()?,
                Some(Token {
                    kind: TokenKind::CloseParenthesis,
                    ..
                })
            ) {
                while let Some(Token {
                    kind: TokenKind::Newline,
                    ..
                }) = lexer.peek_token()?
                {
                    next_token(lexer)?;
                }

                let name_token = next_token(lexer)?;
                if !matches!(name_token.kind, TokenKind::Name(_)) {
                    return Err(ParsingError::ExpectedNameToken { got: name_token });
                }
                let colon_token = expect_token(lexer, TokenKind::Colon)?;
                let typ = parse_expression(lexer)?;
                parameters.push(AstParameter {
                    name_token,
                    colon_token,
                    typ,
                });

                if matches!(
                    lexer.peek_token()?,
                    Some(Token {
                        kind: TokenKind::CloseParenthesis,
                        ..
                    })
                ) {
                    break;
                }
                expect_token(lexer, TokenKind::Comma)?;
                match_token(lexer, &TokenKind::Newline)?;
            }
            let close_parenthesis_token = expect_token(lexer, TokenKind::CloseParenthesis)?;
            let (fat_right_arrow_token, return_type) =
                if let Some(fat_right_arrow) = match_token(lexer, &TokenKind::FatRightArrow)? {
                    let return_type = parse_expression(lexer)?;
                    (Some(fat_right_arrow), Some(return_type))
                } else {
                    (None, None)
                };
            let body = if let Some(do_token) = match_token(lexer, &TokenKind::DoKeyword)? {
                let expression = parse_expression(lexer)?;
                Some(AstBody::DoExpression {
                    do_token,
                    expression: Box::new(expression),
                })
            } else if let Some(Token {
                kind: TokenKind::OpenBrace,
                ..
            }) = lexer.peek_token()?
            {
                Some(AstBody::Block(Box::new(parse_block(lexer)?)))
            } else {
                if name_token.is_some() {
                    return Err(ParsingError::ProcedureWithNameMustHaveBody {
                        location: proc_token.get_location(),
                    });
                }
                None
            };
            Ast::Procedure {
                proc_token,
                name_token,
                open_parenthesis_token,
                parameters,
                close_parenthesis_token,
                fat_right_arrow_token,
                return_type: return_type.map(|return_type| Box::new(return_type)),
                body,
            }
        }
        return_token @ Token {
            kind: TokenKind::ReturnKeyword,
            ..
        } => {
            let value = if !matches!(
                lexer.peek_token()?,
                Some(Token {
                    kind: TokenKind::Newline, // TODO: what if you have `{ return }`, should there be an extra rule here?
                    ..
                })
            ) {
                Some(parse_expression(lexer)?)
            } else {
                None
            };
            Ast::Return {
                return_token,
                value: value.map(|value| Box::new(value)),
            }
        }
        token => return Err(ParsingError::UnexpectedToken { token }),
    };

    // call
    while matches!(
        lexer.peek_token()?,
        Some(Token {
            kind: TokenKind::OpenParenthesis,
            ..
        })
    ) {
        let open_parenthesis_token = expect_token(lexer, TokenKind::OpenParenthesis)?;
        let mut arguments = vec![];
        while !matches!(
            lexer.peek_token()?,
            Some(Token {
                kind: TokenKind::CloseParenthesis,
                ..
            })
        ) {
            while let Some(Token {
                kind: TokenKind::Newline,
                ..
            }) = lexer.peek_token()?
            {
                next_token(lexer)?;
            }

            arguments.push(parse_expression(lexer)?);

            if matches!(
                lexer.peek_token()?,
                Some(Token {
                    kind: TokenKind::CloseParenthesis,
                    ..
                })
            ) {
                break;
            }
            expect_token(lexer, TokenKind::Comma)?;
            match_token(lexer, &TokenKind::Newline)?;
        }
        let close_parenthesis_token = expect_token(lexer, TokenKind::CloseParenthesis)?;
        expression = Ast::Call {
            operand: Box::new(expression),
            open_parenthesis_token,
            arguments,
            close_parenthesis_token,
        };
    }

    Ok(expression)
}

fn parse_binary_expression<'filepath, 'source>(
    lexer: &mut Lexer<'filepath, 'source>,
    parent_precedence: usize,
) -> Result<Ast<'filepath, 'source>, ParsingError<'filepath, 'source>> {
    let mut left = {
        let operator = match lexer.peek_token()? {
            Some(
                plus_token @ Token {
                    kind: TokenKind::Plus,
                    ..
                },
            ) => Some((3, UnaryOperator::Identity { plus_token })),
            Some(
                minus_token @ Token {
                    kind: TokenKind::Minus,
                    ..
                },
            ) => Some((3, UnaryOperator::Negate { minus_token })),
            _ => None,
        };
        if let Some((unary_precedence, operator)) = operator {
            lexer.next_token()?;
            let operand = parse_binary_expression(lexer, unary_precedence)?;
            Ast::Unary {
                operator,
                operand: Box::new(operand),
            }
        } else {
            parse_primary_expression(lexer)?
        }
    };

    loop {
        let (binary_precedence, operator) = {
            match lexer.peek_token()? {
                Some(
                    plus_token @ Token {
                        kind: TokenKind::Plus,
                        ..
                    },
                ) => (1, BinaryOperator::Add { plus_token }),
                Some(
                    minus_token @ Token {
                        kind: TokenKind::Minus,
                        ..
                    },
                ) => (1, BinaryOperator::Subtract { minus_token }),
                Some(
                    asterisk_token @ Token {
                        kind: TokenKind::Asterisk,
                        ..
                    },
                ) => (2, BinaryOperator::Multiply { asterisk_token }),
                Some(
                    slash_token @ Token {
                        kind: TokenKind::Slash,
                        ..
                    },
                ) => (2, BinaryOperator::Divide { slash_token }),
                _ => break,
            }
        };
        if binary_precedence <= parent_precedence {
            break;
        }

        next_token(lexer)?;
        let right = parse_binary_expression(lexer, binary_precedence)?;
        left = Ast::Binary {
            left: Box::new(left),
            operator,
            right: Box::new(right),
        };
    }

    Ok(left)
}

fn parse_block<'filepath, 'source>(
    lexer: &mut Lexer<'filepath, 'source>,
) -> Result<Ast<'filepath, 'source>, ParsingError<'filepath, 'source>> {
    let open_brace_token = expect_token(lexer, TokenKind::OpenBrace)?;
    let mut expressions = vec![];
    while !matches!(
        lexer.peek_token()?,
        Some(Token {
            kind: TokenKind::CloseBrace,
            ..
        })
    ) {
        while let Some(Token {
            kind: TokenKind::Newline,
            ..
        }) = lexer.peek_token()?
        {
            next_token(lexer)?;
        }

        expressions.push(parse_expression(lexer)?);

        expect_token(lexer, TokenKind::Newline)?;
    }
    let close_brace_token = expect_token(lexer, TokenKind::CloseBrace)?;
    Ok(Ast::Block {
        open_brace_token,
        expressions,
        close_brace_token,
    })
}

fn expect_token<'filepath, 'source>(
    lexer: &mut Lexer<'filepath, 'source>,
    expected: TokenKind<'source>,
) -> Result<Token<'filepath, 'source>, ParsingError<'filepath, 'source>> {
    let token = next_token(lexer)?;
    if token.kind != expected {
        return Err(ParsingError::ExpectedTokenButGot {
            expected,
            got: token,
        });
    }
    Ok(token)
}

fn match_token<'filepath, 'source>(
    lexer: &mut Lexer<'filepath, 'source>,
    expected: &TokenKind<'source>,
) -> Result<Option<Token<'filepath, 'source>>, ParsingError<'filepath, 'source>> {
    Ok(
        if let Some(_token) = lexer.peek_token()?.filter(|token| token.kind == *expected) {
            lexer.next_token()?
        } else {
            None
        },
    )
}

fn next_token<'filepath, 'source>(
    lexer: &mut Lexer<'filepath, 'source>,
) -> Result<Token<'filepath, 'source>, ParsingError<'filepath, 'source>> {
    let token = lexer.next_token()?;
    token.ok_or(ParsingError::UnexpectedEndOfFile {
        filepath: lexer.get_filepath(),
    })
}
