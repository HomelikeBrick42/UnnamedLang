use std::rc::Rc;

use crate::{
    error::LexerError,
    source_location::SourceLocation,
    token::{Token, TokenKind},
};

#[derive(Debug, Clone)]
pub struct Lexer {
    pub location: SourceLocation,
    // TODO: Find someway better to peek tokens than cloning the entire lexer
    // There is a Rc for now so we can clone the lexer more cheaply
    pub source: Rc<Vec<char>>,
}

impl Lexer {
    pub fn new(filepath: &str, source: &str) -> Lexer {
        Lexer {
            location: SourceLocation {
                filepath: filepath.to_string(),
                position: 0,
                line: 1,
                column: 1,
            },
            source: Rc::new(source.chars().collect()),
        }
    }

    pub fn next_token(&mut self) -> Result<Token, LexerError> {
        'main: loop {
            while self.current_char().is_whitespace() && self.current_char() != '\n' {
                self.next_char();
            }

            let start_location = self.location.clone();
            let chr = self.next_char();

            if chr == '/' {
                if self.current_char() == '/' {
                    while self.current_char() != '\n' && self.current_char() != '\0' {
                        self.next_char();
                    }
                    continue 'main;
                } else if self.current_char() == '*' {
                    self.next_char();
                    let mut depth = 1usize;
                    while depth > 0 {
                        if self.current_char() == '/' {
                            self.next_char();
                            if self.current_char() == '*' {
                                self.next_char();
                                depth += 1;
                            }
                        } else if self.current_char() == '*' {
                            self.next_char();
                            if self.current_char() == '/' {
                                self.next_char();
                                depth -= 1;
                            }
                        } else {
                            if self.next_char() == '\0' {
                                return Err(LexerError::UnclosedBlockComment {
                                    open_comment_location: start_location,
                                });
                            }
                        }
                    }
                    continue 'main;
                }
            }

            if let Ok(kind) = TokenKind::try_from((chr, self.current_char())) {
                self.next_char();
                return Ok(Token {
                    kind,
                    length: self.location.position - start_location.position,
                    location: start_location,
                });
            }

            if let Ok(kind) = TokenKind::try_from(chr) {
                return Ok(Token {
                    kind,
                    length: self.location.position - start_location.position,
                    location: start_location,
                });
            }

            if chr.is_ascii_digit() {
                self.location = start_location.clone();

                let base = if chr == '0' {
                    match self.next_char() {
                        'b' => {
                            self.next_char();
                            2
                        }

                        'o' => {
                            self.next_char();
                            8
                        }

                        'd' => {
                            self.next_char();
                            10
                        }

                        'x' => {
                            self.next_char();
                            16
                        }

                        _ => 10,
                    }
                } else {
                    10
                };

                fn convert_char_to_digit(
                    base: u128,
                    chr: char,
                    start_location: &SourceLocation,
                    location: &SourceLocation,
                ) -> Result<u128, LexerError> {
                    assert!(base <= 36);
                    let value = match chr {
                        '0'..='9' => chr as u128 - '0' as u128,
                        'A'..='Z' => chr as u128 - 'A' as u128,
                        'a'..='z' => chr as u128 - 'a' as u128,
                        _ => unreachable!(),
                    };
                    if value >= base {
                        Err(LexerError::DigitTooBigForBase {
                            number_location: start_location.clone(),
                            invalid_digit_location: location.clone(),
                            invalid_digit_char: chr,
                            invalid_digit: value,
                        })
                    } else {
                        Ok(value)
                    }
                }

                let mut value = 0;
                while self.current_char().is_ascii_alphanumeric() || self.current_char() == '_' {
                    if self.current_char() == '_' {
                        self.next_char();
                        continue;
                    }

                    value *= base;
                    value += convert_char_to_digit(
                        base,
                        self.next_char(),
                        &start_location,
                        &self.location,
                    )?;
                }

                if self.current_char() == '.' {
                    let mut value = value as f64;
                    let mut discrimiant = 1;
                    while self.current_char().is_ascii_alphanumeric() || self.current_char() == '_'
                    {
                        if self.current_char() == '_' {
                            self.next_char();
                            continue;
                        }

                        discrimiant *= base;
                        value += convert_char_to_digit(
                            base,
                            self.next_char(),
                            &start_location,
                            &self.location,
                        )? as f64
                            / discrimiant as f64;
                    }

                    return Ok(Token {
                        kind: TokenKind::Float(value),
                        length: self.location.position - start_location.position,
                        location: start_location,
                    });
                }

                return Ok(Token {
                    kind: TokenKind::Integer(value),
                    length: self.location.position - start_location.position,
                    location: start_location,
                });
            }

            if chr.is_alphanumeric() || chr == '_' {
                while self.current_char().is_alphanumeric() || self.current_char() == '_' {
                    self.next_char();
                }
                let name: String = self.source[start_location.position..self.location.position]
                    .iter()
                    .collect();
                return Ok(Token {
                    kind: TokenKind::from(name),
                    length: self.location.position - start_location.position,
                    location: start_location,
                });
            }

            if chr == '"' {
                let mut value = String::new();
                while self.current_char() != '"' {
                    let chr = self.next_char();
                    if chr == '\\' {
                        match self.next_char() {
                            '"' => value.push('"'),
                            '\\' => value.push('\\'),
                            'n' => value.push('\n'),
                            chr => {
                                return Err(LexerError::InvalidEscapeChar {
                                    string_location: start_location,
                                    invalid_escape_char_location: self.location.clone(),
                                    invlaid_escape_char: chr,
                                })
                            }
                        }
                    } else if chr == '\0' {
                        return Err(LexerError::UnclosedStringLiteral {
                            open_quote_location: start_location,
                        });
                    } else {
                        value.push(chr);
                    }
                }
                self.next_char();
                return Ok(Token {
                    kind: TokenKind::String(value),
                    length: self.location.position - start_location.position,
                    location: start_location,
                });
            }

            return Err(LexerError::InvalidChar {
                invalid_char_location: start_location,
                invalid_char: chr,
            });
        }
    }

    pub fn peek_token(&self) -> Result<Token, LexerError> {
        let mut copy = self.clone();
        copy.next_token()
    }

    pub fn peek_kind(&self) -> Result<TokenKind, LexerError> {
        Ok(self.peek_token()?.kind)
    }
}

impl Lexer {
    fn current_char(&self) -> char {
        if self.location.position < self.source.len() {
            self.source[self.location.position]
        } else {
            '\0'
        }
    }

    fn next_char(&mut self) -> char {
        let current = self.current_char();
        if current != '\0' {
            self.location.position += 1;
            self.location.column += 1;
            if current == '\n' {
                self.location.line += 1;
                self.location.column = 1;
            }
        }
        current
    }
}
