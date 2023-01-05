use std::str::Chars;

use derive_more::Display;

use super::{SourceLocation, SourceSpan, Token, TokenKind};

#[derive(Debug, Display)]
pub enum LexerError<'filepath> {
    #[display(fmt = "{location}: Unexpected character {character:?}")]
    UnexpectedCharacter {
        location: SourceSpan<'filepath>,
        character: char,
    },
    #[display(fmt = "{location}: Unclosed block comment")]
    UnclosedBlockComment {
        location: SourceSpan<'filepath>,
    },
    InvalidIntegerLiteral {
        location: SourceSpan<'filepath>,
    },
}

#[derive(Clone)]
pub struct Lexer<'filepath, 'source> {
    filepath: &'filepath str,
    source: &'source str,
    chars: Chars<'source>,
    location: SourceLocation,
}

impl<'filepath, 'source> Lexer<'filepath, 'source> {
    pub fn new(filepath: &'filepath str, source: &'source str) -> Self {
        Lexer {
            filepath,
            source,
            chars: source.chars(),
            location: SourceLocation {
                position: 0,
                line: 1,
                column: 1,
            },
        }
    }

    pub fn get_filepath(&self) -> &'filepath str {
        self.filepath
    }

    pub fn next_char(&mut self) -> Option<char> {
        let current = self.chars.next()?;
        self.location.position += current.len_utf8();
        self.location.column += 1;
        if current == '\n' {
            self.location.line += 1;
            self.location.column = 1;
        }
        Some(current)
    }

    pub fn peek_char(&mut self) -> Option<char> {
        self.clone().next_char()
    }

    pub fn next_token(
        &mut self,
    ) -> Result<Option<Token<'filepath, 'source>>, LexerError<'filepath>> {
        loop {
            let start_location = self.location;
            return Ok(Some(Token {
                kind: match self.next_char() {
                    None => return Ok(None),
                    Some('\n') => TokenKind::Newline,
                    Some('/') if self.peek_char() == Some('/') => {
                        while self.peek_char() != Some('\n') {
                            self.next_char();
                        }
                        continue;
                    }
                    Some('/') if self.peek_char() == Some('*') => {
                        self.next_char();
                        let mut depth = 1usize;
                        while let Some(c) = self.next_char() {
                            if c == '/' && self.peek_char() == Some('*') {
                                self.next_char();
                                depth += 1;
                            } else if c == '*' && self.peek_char() == Some('/') {
                                self.next_char();
                                depth -= 1;
                            }
                        }
                        if depth > 0 {
                            return Err(LexerError::UnclosedBlockComment {
                                location: SourceSpan {
                                    filepath: self.filepath,
                                    start: start_location,
                                    end: self.location,
                                },
                            });
                        }
                        continue;
                    }
                    Some('(') => TokenKind::OpenParenthesis,
                    Some(')') => TokenKind::CloseParenthesis,
                    Some('{') => TokenKind::OpenBrace,
                    Some('}') => TokenKind::CloseBrace,
                    Some(',') => TokenKind::Comma,
                    Some(':') => TokenKind::Colon,
                    Some('=') if self.peek_char() == Some('>') => {
                        self.next_char();
                        TokenKind::FatRightArrow
                    }
                    Some('+') => TokenKind::Plus,
                    Some('-') => TokenKind::Minus,
                    Some('*') => TokenKind::Asterisk,
                    Some('/') => TokenKind::Slash,
                    Some('"') => {
                        todo!()
                    }
                    Some(c) if c.is_ascii_digit() => {
                        while let Some(c) = self.peek_char() {
                            if !c.is_ascii_digit() {
                                break;
                            }
                            self.next_char();
                        }
                        let integer = &self.source[start_location.position..self.location.position];
                        TokenKind::Integer(integer.parse().map_err(|_| {
                            LexerError::InvalidIntegerLiteral {
                                location: SourceSpan {
                                    filepath: self.filepath,
                                    start: start_location,
                                    end: self.location,
                                },
                            }
                        })?)
                    }
                    Some(c) if c.is_alphabetic() || c == '_' => {
                        while let Some(c) = self.peek_char() {
                            if !c.is_ascii_alphanumeric() && c != '_' {
                                break;
                            }
                            self.next_char();
                        }
                        let name = &self.source[start_location.position..self.location.position];
                        match name {
                            "proc" => TokenKind::ProcKeyword,
                            "return" => TokenKind::ReturnKeyword,
                            "do" => TokenKind::DoKeyword,
                            "let" => TokenKind::LetKeyword,
                            "var" => TokenKind::VarKeyword,
                            _ => TokenKind::Name(name),
                        }
                    }
                    Some(c) if c.is_whitespace() => continue,
                    Some(c) => {
                        return Err(LexerError::UnexpectedCharacter {
                            location: SourceSpan {
                                filepath: self.filepath,
                                start: start_location,
                                end: self.location,
                            },
                            character: c,
                        });
                    }
                },
                span: SourceSpan {
                    filepath: self.filepath,
                    start: start_location,
                    end: self.location,
                },
            }));
        }
    }

    pub fn peek_token(&self) -> Result<Option<Token<'filepath, 'source>>, LexerError<'filepath>> {
        self.clone().next_token()
    }
}

impl<'filepath, 'source> Iterator for Lexer<'filepath, 'source> {
    type Item = Result<Token<'filepath, 'source>, LexerError<'filepath>>;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_token().transpose()
    }
}
