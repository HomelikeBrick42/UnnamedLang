use std::{collections::HashMap, rc::Rc};

use derive_more::Display;
use enum_as_inner::EnumAsInner;

use crate::{
    Ast, AstAssign, AstAssignDirection, AstBinary, AstCall, AstCast, AstFile, AstIf, AstInteger,
    AstLet, AstName, AstParameter, AstProcedure, AstProcedureBody, AstProcedureType, AstReturn,
    AstScope, AstUnary, AstVar, AstWhile, BinaryOperator, CallingConvention, Lexer, LexerError,
    SourceLocation, SourceSpan, Token, TokenKind, UnaryOperator,
};

#[derive(Debug, Display, EnumAsInner)]
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
    #[display(fmt = "{location}: You can only use #import at file scope")]
    ImportNotAtFileScope { location: SourceSpan },
    #[display(fmt = "{location}: Unable to read '{filepath}': {error}")]
    UnableToReadFile {
        location: SourceSpan,
        filepath: String,
        error: std::io::Error,
    },
    #[display(fmt = "{location}: Cyclic #import of '{filepath}'")]
    CyclicImport {
        location: SourceSpan,
        filepath: String,
    },
    #[display(
        fmt = "{new_location}: Duplicate calling convention directive {new_convention}, the original calling convention was {old_convention}"
    )]
    DuplicateCallingConvention {
        old_convention: CallingConvention,
        new_location: SourceSpan,
        new_convention: CallingConvention,
    },
}

impl From<LexerError> for ParsingError {
    fn from(error: LexerError) -> ParsingError {
        ParsingError::LexerError(error)
    }
}

pub fn parse_file(
    filepath: &str,
    source: &str,
    imported_files: &mut HashMap<String, bool>,
) -> Result<Rc<AstFile>, ParsingError> {
    imported_files.insert(filepath.into(), true);
    let mut lexer = Lexer::new(filepath.into(), source);
    let mut expressions = vec![];
    loop {
        allow_newlines(&mut lexer)?;
        if lexer.peek_token()?.kind == TokenKind::EndOfFile {
            break;
        }
        if lexer.peek_token()?.kind == TokenKind::ImportDirective {
            let import_token = expect_token(&mut lexer, TokenKind::ImportDirective)?;
            let filepath = expect_token(&mut lexer, TokenKind::String)?
                .data
                .into_string()
                .unwrap();
            if let Some(is_parsing) = imported_files.get(&filepath) {
                if *is_parsing {
                    return Err(ParsingError::CyclicImport {
                        location: import_token.location.clone(),
                        filepath,
                    });
                } else {
                    continue;
                }
            }
            let source = std::fs::read_to_string(filepath.clone()).map_err(|error| {
                ParsingError::UnableToReadFile {
                    location: import_token.location,
                    filepath: filepath.clone(),
                    error,
                }
            })?;
            let file = parse_file(&filepath, &source, imported_files)?;
            for expression in &file.expressions {
                expressions.push(expression.clone());
            }
        } else {
            expressions.push(parse_expression(&mut lexer)?);
        }
        expect_newline(&mut lexer)?;
    }
    let end_of_file_token = expect_token(&mut lexer, TokenKind::EndOfFile)?;
    imported_files.insert(filepath.into(), false);
    Ok(AstFile {
        location: SourceSpan::combine_spans(
            &SourceSpan {
                filepath: filepath.into(),
                start: SourceLocation {
                    position: 0,
                    line: 1,
                    column: 1,
                },
                end: SourceLocation {
                    position: 0,
                    line: 1,
                    column: 1,
                },
            },
            &end_of_file_token.location,
        ),
        expressions,
    }
    .into())
}

fn parse_expression(lexer: &mut Lexer) -> Result<Ast, ParsingError> {
    parse_binary_expression(lexer, 0)
}

fn parse_least_expression(lexer: &mut Lexer) -> Result<Ast, ParsingError> {
    parse_binary_expression(lexer, usize::MAX)
}

fn parse_primary_expression(lexer: &mut Lexer) -> Result<Ast, ParsingError> {
    Ok(match lexer.peek_token()?.kind {
        TokenKind::ImportDirective => {
            return Err(ParsingError::ImportNotAtFileScope {
                location: lexer.next_token()?.location,
            })
        }

        TokenKind::Name => {
            let token = expect_token(lexer, TokenKind::Name)?;
            Ast::Name(
                AstName {
                    location: token.location,
                    name: token.data.into_string().unwrap(),
                }
                .into(),
            )
        }

        TokenKind::Integer => {
            let token = expect_token(lexer, TokenKind::Integer)?;
            Ast::Integer(
                AstInteger {
                    location: token.location,
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
            let proc_token = expect_token(lexer, TokenKind::ProcKeyword)?;
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
                let return_type = parse_least_expression(lexer)?;
                let mut calling_convention = None;
                while matches!(
                    lexer.peek_token()?.kind,
                    TokenKind::CDeclDirective
                        | TokenKind::StdCallDirective
                        | TokenKind::FastCallDirective
                ) {
                    let directive = lexer.next_token()?;
                    match directive.kind {
                        TokenKind::CDeclDirective => {
                            if let Some(old) = calling_convention {
                                return Err(ParsingError::DuplicateCallingConvention {
                                    old_convention: old,
                                    new_location: directive.location,
                                    new_convention: CallingConvention::CDecl,
                                });
                            }
                            calling_convention = CallingConvention::CDecl.into();
                        }
                        TokenKind::StdCallDirective => {
                            if let Some(old) = calling_convention {
                                return Err(ParsingError::DuplicateCallingConvention {
                                    old_convention: old,
                                    new_location: directive.location,
                                    new_convention: CallingConvention::StdCall,
                                });
                            }
                            calling_convention = CallingConvention::StdCall.into();
                        }
                        TokenKind::FastCallDirective => {
                            if let Some(old) = calling_convention {
                                return Err(ParsingError::DuplicateCallingConvention {
                                    old_convention: old,
                                    new_location: directive.location,
                                    new_convention: CallingConvention::FastCall,
                                });
                            }
                            calling_convention = CallingConvention::FastCall.into();
                        }
                        _ => unreachable!(),
                    }
                }
                Ast::ProcedureType(
                    AstProcedureType {
                        location: SourceSpan::combine_spans(
                            &proc_token.location,
                            &return_type.get_location(),
                        ),
                        parameter_types,
                        return_type,
                        calling_convention: calling_convention.unwrap_or(CallingConvention::CDecl),
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
                    let mutable = if lexer.peek_token()?.kind == TokenKind::VarKeyword {
                        expect_token(lexer, TokenKind::VarKeyword)?;
                        true
                    } else {
                        false
                    };
                    let name_token = expect_token(lexer, TokenKind::Name)?;
                    expect_token(lexer, TokenKind::Colon)?;
                    let typ = parse_least_expression(lexer)?;
                    parameters.push(
                        AstParameter {
                            location: SourceSpan::combine_spans(
                                &name_token.location,
                                &typ.get_location(),
                            ),
                            mutable,
                            name: name_token.data.into_string().unwrap(),
                            typ,
                        }
                        .into(),
                    );
                    expect_comma_and_or_newline(lexer)?;
                }
                expect_token(lexer, TokenKind::CloseParenthesis)?;
                expect_token(lexer, TokenKind::FatRightArrow)?;
                let return_type = parse_least_expression(lexer)?;
                let mut calling_convention = None;
                while matches!(
                    lexer.peek_token()?.kind,
                    TokenKind::CDeclDirective
                        | TokenKind::StdCallDirective
                        | TokenKind::FastCallDirective
                ) {
                    let directive = lexer.next_token()?;
                    match directive.kind {
                        TokenKind::CDeclDirective => {
                            if let Some(old) = calling_convention {
                                return Err(ParsingError::DuplicateCallingConvention {
                                    old_convention: old,
                                    new_location: directive.location,
                                    new_convention: CallingConvention::CDecl,
                                });
                            }
                            calling_convention = CallingConvention::CDecl.into();
                        }
                        TokenKind::StdCallDirective => {
                            if let Some(old) = calling_convention {
                                return Err(ParsingError::DuplicateCallingConvention {
                                    old_convention: old,
                                    new_location: directive.location,
                                    new_convention: CallingConvention::StdCall,
                                });
                            }
                            calling_convention = CallingConvention::StdCall.into();
                        }
                        TokenKind::FastCallDirective => {
                            if let Some(old) = calling_convention {
                                return Err(ParsingError::DuplicateCallingConvention {
                                    old_convention: old,
                                    new_location: directive.location,
                                    new_convention: CallingConvention::FastCall,
                                });
                            }
                            calling_convention = CallingConvention::FastCall.into();
                        }
                        _ => unreachable!(),
                    }
                }
                let (body, body_location) =
                    if lexer.peek_token()?.kind == TokenKind::ExternDirective {
                        let extern_token = expect_token(lexer, TokenKind::ExternDirective)?;
                        let extern_name_token = expect_token(lexer, TokenKind::String)?;
                        (
                            AstProcedureBody::ExternName(
                                extern_name_token.data.as_string().unwrap().clone(),
                            ),
                            SourceSpan::combine_spans(
                                &extern_token.location,
                                &extern_name_token.location,
                            ),
                        )
                    } else {
                        let scope = parse_scope(lexer)?;
                        (
                            AstProcedureBody::Scope(scope.clone()),
                            scope.location.clone(),
                        )
                    };
                Ast::Procedure(
                    AstProcedure {
                        location: SourceSpan::combine_spans(&proc_token.location, &body_location),
                        name,
                        parameters,
                        return_type,
                        calling_convention: calling_convention
                            .unwrap_or(crate::CallingConvention::CDecl),
                        body,
                    }
                    .into(),
                )
            }
        }

        TokenKind::OpenBrace => Ast::Scope(parse_scope(lexer)?),

        TokenKind::ReturnKeyword => {
            let return_keyword = expect_token(lexer, TokenKind::ReturnKeyword)?;
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
                    location: value
                        .as_ref()
                        .map(|value| {
                            SourceSpan::combine_spans(
                                &return_keyword.location,
                                &value.get_location(),
                            )
                        })
                        .unwrap_or_else(|| return_keyword.location.clone()),
                    value,
                }
                .into(),
            )
        }

        TokenKind::IfKeyword => {
            let if_token = expect_token(lexer, TokenKind::IfKeyword)?;
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
                    location: SourceSpan::combine_spans(
                        &if_token.location,
                        &else_expression
                            .as_ref()
                            .map(|else_expression| else_expression.get_location())
                            .unwrap_or_else(|| then_expression.get_location()),
                    ),
                    condition,
                    then_expression,
                    else_expression,
                }
                .into(),
            )
        }

        TokenKind::WhileKeyword => {
            let while_token = expect_token(lexer, TokenKind::WhileKeyword)?;
            let condition = parse_expression(lexer)?;
            let then_expression = parse_expression(lexer)?;
            Ast::While(
                AstWhile {
                    location: SourceSpan::combine_spans(
                        &while_token.location,
                        &then_expression.get_location(),
                    ),
                    condition,
                    then_expression,
                }
                .into(),
            )
        }

        TokenKind::LetKeyword => {
            let let_token = expect_token(lexer, TokenKind::LetKeyword)?;
            let name = expect_token(lexer, TokenKind::Name)?
                .data
                .into_string()
                .unwrap();
            let typ = if lexer.peek_token()?.kind == TokenKind::Colon {
                expect_token(lexer, TokenKind::Colon)?;
                Some(parse_least_expression(lexer)?)
            } else {
                None
            };
            expect_token(lexer, TokenKind::Equal)?;
            let value = parse_expression(lexer)?;
            Ast::LetDeclaration(
                AstLet {
                    location: SourceSpan::combine_spans(&let_token.location, &value.get_location()),
                    name,
                    typ,
                    value,
                }
                .into(),
            )
        }

        TokenKind::VarKeyword => {
            let var_token = expect_token(lexer, TokenKind::VarKeyword)?;
            let name = expect_token(lexer, TokenKind::Name)?
                .data
                .into_string()
                .unwrap();
            let typ = if lexer.peek_token()?.kind == TokenKind::Colon {
                expect_token(lexer, TokenKind::Colon)?;
                Some(parse_least_expression(lexer)?)
            } else {
                None
            };
            expect_token(lexer, TokenKind::LeftArrow)?;
            let value = parse_expression(lexer)?;
            Ast::VarDeclaration(
                AstVar {
                    location: SourceSpan::combine_spans(&var_token.location, &value.get_location()),
                    name,
                    typ,
                    value,
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
    let open_brace_token = expect_token(lexer, TokenKind::OpenBrace)?;
    let mut expressions = vec![];
    loop {
        allow_newlines(lexer)?;
        if lexer.peek_token()?.kind == TokenKind::CloseBrace {
            break;
        }
        expressions.push(parse_expression(lexer)?);
        expect_newline(lexer)?;
    }
    let close_brace_token = expect_token(lexer, TokenKind::CloseBrace)?;
    Ok(AstScope {
        location: SourceSpan::combine_spans(
            &open_brace_token.location,
            &close_brace_token.location,
        ),
        expressions,
    }
    .into())
}

fn parse_binary_expression(
    lexer: &mut Lexer,
    parent_precedence: usize,
) -> Result<Ast, ParsingError> {
    fn is_unary_operator(kind: TokenKind) -> bool {
        matches!(
            kind,
            TokenKind::Plus | TokenKind::Minus | TokenKind::Caret | TokenKind::Ampersand
        )
    }

    fn get_binary_precedence(kind: TokenKind) -> usize {
        match kind {
            TokenKind::Asterisk | TokenKind::Slash | TokenKind::Percent => 3,
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

    let mut left = if lexer.peek_token()?.kind == TokenKind::CastKeyword {
        let cast_token = expect_token(lexer, TokenKind::CastKeyword)?;
        expect_token(lexer, TokenKind::OpenParenthesis)?;
        let typ = parse_expression(lexer)?;
        expect_token(lexer, TokenKind::CloseParenthesis)?;
        let operand = parse_least_expression(lexer)?;
        Ast::Cast(
            AstCast {
                location: SourceSpan::combine_spans(&cast_token.location, &operand.get_location()),
                typ,
                operand,
            }
            .into(),
        )
    } else {
        if is_unary_operator(lexer.peek_token()?.kind) {
            let operator_token = lexer.next_token()?;
            let operator = match &operator_token.kind {
                TokenKind::Plus => UnaryOperator::Identity,
                TokenKind::Minus => UnaryOperator::Negation,
                TokenKind::Caret => UnaryOperator::PointerType,
                TokenKind::Ampersand => UnaryOperator::AddressOf,
                _ => unreachable!(),
            };
            let operand = parse_least_expression(lexer)?;
            Ast::Unary(
                AstUnary {
                    location: SourceSpan::combine_spans(
                        &operator_token.location,
                        &operand.get_location(),
                    ),
                    operator,
                    operand,
                }
                .into(),
            )
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
                let close_parenthesis_token = expect_token(lexer, TokenKind::CloseParenthesis)?;
                Ast::Call(
                    AstCall {
                        location: SourceSpan::combine_spans(
                            &left.get_location(),
                            &close_parenthesis_token.location,
                        ),
                        operand: left,
                        arguments,
                    }
                    .into(),
                )
            }

            TokenKind::LeftArrow if parent_precedence == 0 => {
                expect_token(lexer, TokenKind::LeftArrow)?;
                allow_newline(lexer)?;
                let value = parse_expression(lexer)?;
                Ast::Assign(
                    AstAssign {
                        location: SourceSpan::combine_spans(
                            &left.get_location(),
                            &value.get_location(),
                        ),
                        direction: AstAssignDirection::Left,
                        operand: left,
                        value,
                    }
                    .into(),
                )
            }

            TokenKind::RightArrow if parent_precedence == 0 => {
                expect_token(lexer, TokenKind::RightArrow)?;
                allow_newline(lexer)?;
                let operand = parse_expression(lexer)?;
                Ast::Assign(
                    AstAssign {
                        location: SourceSpan::combine_spans(
                            &left.get_location(),
                            &operand.get_location(),
                        ),
                        direction: AstAssignDirection::Right,
                        operand,
                        value: left,
                    }
                    .into(),
                )
            }

            TokenKind::Caret => {
                let caret_token = expect_token(lexer, TokenKind::Caret)?;
                Ast::Unary(
                    AstUnary {
                        location: SourceSpan::combine_spans(
                            &left.get_location(),
                            &caret_token.location,
                        ),
                        operator: UnaryOperator::Dereference,
                        operand: left,
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
                    TokenKind::Percent => BinaryOperator::Remainder,
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
                        location: SourceSpan::combine_spans(
                            &left.get_location(),
                            &right.get_location(),
                        ),
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
