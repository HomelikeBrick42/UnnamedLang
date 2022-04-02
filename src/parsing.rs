use std::rc::Rc;

use crate::{
    ast::{Ast, Parameter},
    error::{LexerError, ParsingError},
    lexer::Lexer,
    token::{Token, TokenKind},
};

pub fn parse_file(filepath: &str, source: &str) -> Result<Rc<Ast>, ParsingError> {
    let mut lexer = Lexer::new(filepath, source);
    let mut expressions = vec![];
    while lexer.peek_kind()? != TokenKind::EndOfFile {
        allow_multiple_newlines(&mut lexer)?;
        if lexer.peek_kind()? == TokenKind::EndOfFile {
            break;
        }
        expressions.push(parse(&mut lexer)?);
        expect_newline(&mut lexer)?;
    }
    let end_of_file_token = lexer.next_token()?;
    assert_eq!(end_of_file_token.kind, TokenKind::EndOfFile);
    Ok(Rc::new(Ast::File {
        expressions,
        end_of_file_token,
    }))
}

fn parse(lexer: &mut Lexer) -> Result<Rc<Ast>, ParsingError> {
    _parse_binary_expression(lexer, 0)
}

fn parse_least_expression(lexer: &mut Lexer) -> Result<Rc<Ast>, ParsingError> {
    _parse_binary_expression(lexer, usize::MAX)
}

fn parse_block(lexer: &mut Lexer) -> Result<Rc<Ast>, ParsingError> {
    let open_brace_token = expect_token(lexer, TokenKind::OpenBrace)?;
    let mut expressions = vec![];
    while lexer.peek_kind()? != TokenKind::CloseBrace {
        allow_multiple_newlines(lexer)?;
        expressions.push(parse(lexer)?);
        if lexer.peek_kind()? == TokenKind::CloseBrace {
            break;
        }
        expect_newline(lexer)?;
    }
    let close_brace_token = expect_token(lexer, TokenKind::CloseBrace)?;
    Ok(Rc::new(Ast::Block {
        open_brace_token,
        expressions,
        close_brace_token,
    }))
}

fn _parse_primary_expression(lexer: &mut Lexer) -> Result<Rc<Ast>, ParsingError> {
    Ok(Rc::new(match lexer.peek_kind()? {
        TokenKind::Name(_) => {
            let name_token = lexer.next_token()?;
            if lexer.peek_kind()? == TokenKind::Colon {
                let colon_token = expect_token(lexer, TokenKind::Colon)?;
                let typ = parse_least_expression(lexer)?;
                Ast::Declaration {
                    name_or_wildcard_token: name_token,
                    colon_token,
                    typ,
                }
            } else {
                Ast::Name(name_token)
            }
        }

        TokenKind::String(_) => Ast::String(lexer.next_token()?),
        TokenKind::Float(_) => Ast::Float(lexer.next_token()?),
        TokenKind::Integer(_) => Ast::Integer(lexer.next_token()?),
        TokenKind::Wildcard => {
            let wildcard_token = lexer.next_token()?;
            if lexer.peek_kind()? == TokenKind::Colon {
                let colon_token = expect_token(lexer, TokenKind::Colon)?;
                let typ = parse_least_expression(lexer)?;
                Ast::Declaration {
                    name_or_wildcard_token: wildcard_token,
                    colon_token,
                    typ,
                }
            } else {
                Ast::Wildcard(wildcard_token)
            }
        }

        TokenKind::Const => {
            let const_token = lexer.next_token()?;
            let name_or_wildcard_token = lexer.next_token()?;
            if !name_or_wildcard_token.kind.is_name() && !name_or_wildcard_token.kind.is_wildcard()
            {
                return Err(ParsingError::ExpectedNameOrWildcard {
                    got: name_or_wildcard_token,
                });
            }

            let (open_square_bracket_token, generic_parameters, close_square_bracket_token) =
                if lexer.peek_kind()? == TokenKind::OpenSquareBracket {
                    let open_square_bracket_token = lexer.next_token()?;
                    allow_newline(lexer)?;
                    let mut generic_parameters = vec![];
                    while lexer.peek_kind()? != TokenKind::CloseSquareBracket {
                        let name_or_wildcard_token = lexer.next_token()?;
                        if !name_or_wildcard_token.kind.is_name()
                            && !name_or_wildcard_token.kind.is_wildcard()
                        {
                            return Err(ParsingError::ExpectedNameOrWildcard {
                                got: name_or_wildcard_token,
                            });
                        }
                        let colon_token = expect_token(lexer, TokenKind::Colon)?;
                        let typ = parse(lexer)?;
                        generic_parameters.push(Parameter {
                            name_or_wildcard_token,
                            colon_token,
                            typ,
                        });
                        if lexer.peek_kind()? == TokenKind::CloseSquareBracket {
                            break;
                        }
                        expect_comma_or_newline(lexer)?;
                    }
                    let close_square_bracket_token =
                        expect_token(lexer, TokenKind::CloseSquareBracket)?;
                    (
                        Some(open_square_bracket_token),
                        Some(generic_parameters),
                        Some(close_square_bracket_token),
                    )
                } else {
                    (None, None, None)
                };

            let (colon_token, typ) = if lexer.peek_kind()? == TokenKind::Colon {
                let colon_token = lexer.next_token()?;
                let typ = parse(lexer)?;
                (Some(colon_token), Some(typ))
            } else {
                (None, None)
            };

            let equals_token = expect_token(lexer, TokenKind::Equals)?;
            let value = parse(lexer)?;
            Ast::ConstDeclaration {
                const_token,
                name_or_wildcard_token,
                open_square_bracket_token,
                generic_parameters,
                close_square_bracket_token,
                colon_token,
                typ,
                equals_token,
                value,
            }
        }

        TokenKind::Func => {
            let func_token = lexer.next_token()?;
            let open_parenthesis_token = expect_token(lexer, TokenKind::OpenParenthesis)?;
            allow_newline(lexer)?;
            let mut parameters = vec![];
            while lexer.peek_kind()? != TokenKind::CloseParenthesis {
                let name_or_wildcard_token = lexer.next_token()?;
                if !name_or_wildcard_token.kind.is_name()
                    && !name_or_wildcard_token.kind.is_wildcard()
                {
                    return Err(ParsingError::ExpectedNameOrWildcard {
                        got: name_or_wildcard_token,
                    });
                }
                let colon_token = expect_token(lexer, TokenKind::Colon)?;
                let typ = parse(lexer)?;
                parameters.push(Parameter {
                    name_or_wildcard_token,
                    colon_token,
                    typ,
                });
                if lexer.peek_kind()? == TokenKind::CloseParenthesis {
                    break;
                }
                expect_comma_or_newline(lexer)?;
            }
            let close_parenthesis_token = expect_token(lexer, TokenKind::CloseParenthesis)?;
            let colon_token = expect_token(lexer, TokenKind::Colon)?;
            let typ = parse_least_expression(lexer)?;
            let body = if lexer.peek_kind()? == TokenKind::OpenBrace {
                Some(parse_block(lexer)?)
            } else {
                None
            };
            Ast::Function {
                func_token,
                open_parenthesis_token,
                parameters,
                close_parenthesis_token,
                colon_token,
                typ,
                body,
            }
        }

        TokenKind::Proc => {
            let proc_token = lexer.next_token()?;
            let open_parenthesis_token = expect_token(lexer, TokenKind::OpenParenthesis)?;
            allow_newline(lexer)?;
            let mut parameters = vec![];
            while lexer.peek_kind()? != TokenKind::CloseParenthesis {
                let name_or_wildcard_token = lexer.next_token()?;
                if !name_or_wildcard_token.kind.is_name()
                    && !name_or_wildcard_token.kind.is_wildcard()
                {
                    return Err(ParsingError::ExpectedNameOrWildcard {
                        got: name_or_wildcard_token,
                    });
                }
                let colon_token = expect_token(lexer, TokenKind::Colon)?;
                let typ = parse(lexer)?;
                parameters.push(Parameter {
                    name_or_wildcard_token,
                    colon_token,
                    typ,
                });
                if lexer.peek_kind()? == TokenKind::CloseParenthesis {
                    break;
                }
                expect_comma_or_newline(lexer)?;
            }
            let close_parenthesis_token = expect_token(lexer, TokenKind::CloseParenthesis)?;
            let colon_token = expect_token(lexer, TokenKind::Colon)?;
            let typ = parse_least_expression(lexer)?;
            let body = if lexer.peek_kind()? == TokenKind::OpenBrace {
                Some(parse_block(lexer)?)
            } else {
                None
            };
            Ast::Procedure {
                proc_token,
                open_parenthesis_token,
                parameters,
                close_parenthesis_token,
                colon_token,
                typ,
                body,
            }
        }

        TokenKind::Return => {
            let return_token = lexer.next_token()?;
            let value = parse(lexer)?;
            Ast::Return {
                return_token,
                value,
            }
        }

        TokenKind::If => {
            let if_token = lexer.next_token()?;
            let condition = parse(lexer)?;
            let then_block = parse_block(lexer)?;
            let (else_token, else_block) = if lexer.peek_kind()? == TokenKind::Else {
                let else_token = lexer.next_token()?;
                let else_block = parse_block(lexer)?;
                (Some(else_token), Some(else_block))
            } else {
                (None, None)
            };
            Ast::If {
                if_token,
                condition,
                then_block,
                else_token,
                else_block,
            }
        }

        TokenKind::OpenParenthesis => {
            let open_parenthesis_token = lexer.next_token()?;
            let expression = parse(lexer)?;
            let close_parenthesis_token = expect_token(lexer, TokenKind::CloseParenthesis)?;
            Ast::ParenthesisedExpression {
                open_parenthesis_token,
                expression,
                close_parenthesis_token,
            }
        }

        _ => {
            return Err(ParsingError::ExpectedExpression {
                got: lexer.next_token()?,
            })
        }
    }))
}

fn _parse_binary_expression(
    lexer: &mut Lexer,
    parent_precedence: usize,
) -> Result<Rc<Ast>, ParsingError> {
    let unary_precedence = get_unary_precedence(lexer.peek_kind()?);
    let mut left = if unary_precedence > 0 {
        let operator_token = lexer.next_token()?;
        allow_newline(lexer)?;
        let operand = _parse_binary_expression(lexer, unary_precedence)?;
        Rc::new(Ast::Unary {
            operator_token,
            operand,
        })
    } else {
        _parse_primary_expression(lexer)?
    };

    loop {
        left = Rc::new(match lexer.peek_kind()? {
            TokenKind::OpenParenthesis => {
                let open_parenthesis_token = lexer.next_token()?;
                allow_newline(lexer)?;
                let mut arguments = vec![];
                while lexer.peek_kind()? != TokenKind::CloseParenthesis {
                    arguments.push(parse(lexer)?);
                    if lexer.peek_kind()? == TokenKind::CloseParenthesis {
                        break;
                    }
                    expect_comma_or_newline(lexer)?;
                }
                let close_parenthesis_token = expect_token(lexer, TokenKind::CloseParenthesis)?;
                Ast::Call {
                    operand: left,
                    open_parenthesis_token,
                    arguments,
                    close_parenthesis_token,
                }
            }

            TokenKind::OpenSquareBracket => {
                let open_square_bracket_token = lexer.next_token()?;
                allow_newline(lexer)?;
                let mut arguments = vec![];
                while lexer.peek_kind()? != TokenKind::CloseSquareBracket {
                    arguments.push(parse(lexer)?);
                    if lexer.peek_kind()? == TokenKind::CloseSquareBracket {
                        break;
                    }
                    expect_comma_or_newline(lexer)?;
                }
                let close_square_bracket_token =
                    expect_token(lexer, TokenKind::CloseSquareBracket)?;
                Ast::GenericInstantiation {
                    operand: left,
                    open_square_bracket_token,
                    arguments,
                    close_square_bracket_token,
                }
            }

            _ => {
                let binary_precedence = get_binary_precedence(lexer.peek_kind()?);
                if binary_precedence <= parent_precedence {
                    break;
                }

                let operator_token = lexer.next_token()?;
                allow_newline(lexer)?;
                let right = _parse_binary_expression(lexer, binary_precedence)?;
                Ast::Binary {
                    left,
                    operator_token,
                    right,
                }
            }
        })
    }

    Ok(left)
}

fn get_unary_precedence(kind: TokenKind) -> usize {
    match kind {
        TokenKind::Plus | TokenKind::Minus => 4,
        _ => 0,
    }
}

fn get_binary_precedence(kind: TokenKind) -> usize {
    match kind {
        TokenKind::Asterisk | TokenKind::Slash | TokenKind::Percent => 3,
        TokenKind::Plus | TokenKind::Minus => 2,
        TokenKind::LeftArrow | TokenKind::RightArrow => 1,
        _ => 0,
    }
}

fn expect_token(lexer: &mut Lexer, kind: TokenKind) -> Result<Token, ParsingError> {
    let token = lexer.next_token()?;
    if token.kind == kind {
        Ok(token)
    } else {
        Err(ParsingError::ExpectedToken {
            expected: kind,
            got: token,
        })
    }
}

fn expect_comma_or_newline(lexer: &mut Lexer) -> Result<(), ParsingError> {
    let token = lexer.next_token()?;
    if token.kind != TokenKind::Comma && token.kind != TokenKind::Newline {
        Err(ParsingError::ExpectedCommaOrNewline { got: token })
    } else {
        if token.kind == TokenKind::Comma {
            allow_newline(lexer)?;
        }
        Ok(())
    }
}

fn expect_newline(lexer: &mut Lexer) -> Result<(), ParsingError> {
    let token = lexer.next_token()?;
    if token.kind != TokenKind::Newline && token.kind != TokenKind::EndOfFile {
        Err(ParsingError::ExpectedToken {
            expected: TokenKind::Newline,
            got: token,
        })
    } else {
        Ok(())
    }
}

fn allow_newline(lexer: &mut Lexer) -> Result<(), LexerError> {
    if lexer.peek_kind()? == TokenKind::Newline {
        lexer.next_token()?;
    }
    Ok(())
}

fn allow_multiple_newlines(lexer: &mut Lexer) -> Result<(), LexerError> {
    while lexer.peek_kind()? == TokenKind::Newline {
        lexer.next_token()?;
    }
    Ok(())
}
