use std::rc::Rc;

use phf::phf_map;

use derive_more::Display;
use enum_as_inner::EnumAsInner;

use crate::{SourceLocation, SourceSpan, Token, TokenData, TokenKind};

#[derive(Clone, PartialEq, Debug, Display, EnumAsInner)]
pub enum LexerError {
    #[display(fmt = "{}: Unexpected character '{}'", location, chr)]
    UnexpectedChar { location: SourceSpan, chr: char },
    #[display(fmt = "{}: Digit '{}' is too big for base '{}'", location, chr, base)]
    DigitTooBigForBase {
        location: SourceSpan,
        chr: char,
        base: u128,
    },
    #[display(fmt = "{}: Unknown directive '#{}'", location, name)]
    UnknownDirective { location: SourceSpan, name: String },
}

#[derive(Clone)]
pub struct Lexer {
    filepath: String,
    pub location: SourceLocation,
    source: Rc<Vec<char>>,
}

static KEYWORDS: phf::Map<&'static str, TokenKind> = phf_map! {
    "proc" => TokenKind::ProcKeyword,
    "return" => TokenKind::ReturnKeyword,
    "let" => TokenKind::LetKeyword,
    "var" => TokenKind::VarKeyword,
    "if" => TokenKind::IfKeyword,
    "else" => TokenKind::ElseKeyword,
    "while" => TokenKind::WhileKeyword,
    "cast" => TokenKind::CastKeyword,
};

static DIRECTIVES: phf::Map<&'static str, TokenKind> = phf_map! {
    "import" => TokenKind::ImportDirective,
    "extern" => TokenKind::ExternDirective,
    "cdecl" => TokenKind::CDeclDirective,
    "stdcall" => TokenKind::StdCallDirective,
    "fastcall" => TokenKind::FastCallDirective,
};

static SINGLE_CHAR_TOKENS: phf::Map<char, TokenKind> = phf_map! {
    '\0' => TokenKind::EndOfFile,
    '\n' => TokenKind::Newline,
    '(' => TokenKind::OpenParenthesis,
    ')' => TokenKind::CloseParenthesis,
    '{' => TokenKind::OpenBrace,
    '}' => TokenKind::CloseBrace,
    ':' => TokenKind::Colon,
    ',' => TokenKind::Comma,
    '^' => TokenKind::Caret,
    '&' => TokenKind::Ampersand,
    '=' => TokenKind::Equal,
    '+' => TokenKind::Plus,
    '-' => TokenKind::Minus,
    '*' => TokenKind::Asterisk,
    '/' => TokenKind::Slash,
    '%' => TokenKind::Percent,
    '<' => TokenKind::LessThan,
    '>' => TokenKind::GreaterThan,
};

static DOUBLE_CHAR_TOKENS: phf::Map<char, phf::Map<char, TokenKind>> = phf_map! {
    '\n' => phf_map! {
        '\r' => TokenKind::Newline,
    },
    '\r' => phf_map! {
        '\n' => TokenKind::Newline,
    },
    '=' => phf_map! {
        '=' => TokenKind::EqualEqual,
        '>' => TokenKind::FatRightArrow,
    },
    '!' => phf_map! {
        '=' => TokenKind::ExclamationMarkEqual,
    },
    '<' => phf_map! {
        '=' => TokenKind::LessThanEqual,
        '-' => TokenKind::LeftArrow,
    },
    '>' => phf_map! {
        '=' => TokenKind::GreaterThanEqual,
    },
    '-' =>phf_map! {
        '>' => TokenKind::RightArrow,
    },
};

impl Lexer {
    pub fn new(filepath: String, source: &str) -> Lexer {
        Lexer {
            filepath,
            location: SourceLocation {
                position: 0,
                line: 1,
                column: 1,
            },
            source: Rc::new(source.chars().collect()),
        }
    }

    pub fn next_token(&mut self) -> Result<Token, LexerError> {
        'whitespace_loop: loop {
            if self.peek_char() == ' ' || self.peek_char() == '\t' {
                self.next_char();
                continue 'whitespace_loop;
            }

            if self.peek_char() == '/' {
                let old_location = self.location.clone();
                self.next_char();
                if self.peek_char() == '*' {
                    self.next_char();
                    let mut depth = 1;
                    while depth != 0 {
                        let chr = self.next_char();
                        if chr == '*' && self.peek_char() == '/' {
                            self.next_char();
                            depth -= 1;
                        } else if chr == '/' && self.peek_char() == '*' {
                            self.next_char();
                            depth += 1;
                        }
                    }
                    continue 'whitespace_loop;
                } else if self.peek_char() == '/' {
                    self.next_char();
                    while self.peek_char() != '\r' && self.peek_char() != '\n' {
                        self.next_char();
                    }
                    continue 'whitespace_loop;
                } else {
                    self.location = old_location;
                }
            }

            break 'whitespace_loop;
        }

        let start_location = self.location.clone();
        if self.peek_char().is_alphabetic() || self.peek_char() == '_' {
            let mut name = String::new();
            while self.peek_char().is_alphanumeric() || self.peek_char() == '_' {
                name.push(self.next_char());
            }
            if KEYWORDS.contains_key(&name) {
                Ok(Token {
                    kind: KEYWORDS.get(&name).unwrap().clone(),
                    location: SourceSpan {
                        filepath: self.filepath.clone(),
                        start: start_location,
                        end: self.location.clone(),
                    },
                    data: TokenData::None,
                })
            } else {
                Ok(Token {
                    kind: TokenKind::Name,
                    location: SourceSpan {
                        filepath: self.filepath.clone(),
                        start: start_location,
                        end: self.location.clone(),
                    },
                    data: TokenData::String(name),
                })
            }
        } else if self.peek_char() == '"' {
            self.next_char();
            let mut string = String::new();
            while self.peek_char() != '\0' && self.peek_char() != '"' {
                string.push(self.next_char());
            }
            assert_eq!(self.next_char(), '"');
            Ok(Token {
                kind: TokenKind::String,
                location: SourceSpan {
                    filepath: self.filepath.clone(),
                    start: start_location,
                    end: self.location.clone(),
                },
                data: TokenData::String(string),
            })
        } else if self.peek_char() == '#' {
            self.next_char();
            let mut name = String::new();
            while self.peek_char().is_alphanumeric() || self.peek_char() == '_' {
                name.push(self.next_char());
            }
            if DIRECTIVES.contains_key(&name) {
                Ok(Token {
                    kind: DIRECTIVES.get(&name).unwrap().clone(),
                    location: SourceSpan {
                        filepath: self.filepath.clone(),
                        start: start_location,
                        end: self.location.clone(),
                    },
                    data: TokenData::None,
                })
            } else {
                Err(LexerError::UnknownDirective {
                    location: SourceSpan {
                        filepath: self.filepath.clone(),
                        start: start_location,
                        end: self.location.clone(),
                    },
                    name,
                })
            }
        } else if self.peek_char().is_ascii_digit() {
            let base = 10;
            let mut value = 0;
            'parse_integer: while self.peek_char().is_ascii_alphanumeric()
                || self.peek_char() == '_'
            {
                let chr_location = self.location.clone();
                let chr = self.next_char();
                let digit_value = match chr {
                    '0'..='9' => chr as u128 - '0' as u128,
                    'A'..='Z' => chr as u128 - 'A' as u128 + 10,
                    'a'..='z' => chr as u128 - 'a' as u128 + 10,
                    '_' => continue 'parse_integer,
                    _ => unreachable!(),
                };
                if digit_value >= base {
                    return Err(LexerError::DigitTooBigForBase {
                        location: SourceSpan {
                            filepath: self.filepath.clone(),
                            start: chr_location,
                            end: self.location.clone(),
                        },
                        chr,
                        base,
                    });
                }
                value *= base;
                value += digit_value;
            }
            Ok(Token {
                kind: TokenKind::Integer,
                location: SourceSpan {
                    filepath: self.filepath.clone(),
                    start: start_location,
                    end: self.location.clone(),
                },
                data: TokenData::Integer(value),
            })
        } else {
            let chr = self.next_char();
            if DOUBLE_CHAR_TOKENS.contains_key(&chr)
                && DOUBLE_CHAR_TOKENS
                    .get(&chr)
                    .unwrap()
                    .contains_key(&self.peek_char())
            {
                let second_char = self.next_char();
                Ok(Token {
                    kind: DOUBLE_CHAR_TOKENS
                        .get(&chr)
                        .unwrap()
                        .get(&second_char)
                        .unwrap()
                        .clone(),
                    location: SourceSpan {
                        filepath: self.filepath.clone(),
                        start: start_location,
                        end: self.location.clone(),
                    },
                    data: TokenData::None,
                })
            } else if SINGLE_CHAR_TOKENS.contains_key(&chr) {
                Ok(Token {
                    kind: SINGLE_CHAR_TOKENS.get(&chr).unwrap().clone(),
                    location: SourceSpan {
                        filepath: self.filepath.clone(),
                        start: start_location,
                        end: self.location.clone(),
                    },
                    data: TokenData::None,
                })
            } else {
                Err(LexerError::UnexpectedChar {
                    location: SourceSpan {
                        filepath: self.filepath.clone(),
                        start: start_location,
                        end: self.location.clone(),
                    },
                    chr,
                })
            }
        }
    }

    pub fn peek_token(&self) -> Result<Token, LexerError> {
        let mut copy = self.clone();
        copy.next_token()
    }

    fn next_char(&mut self) -> char {
        let current = self.peek_char();
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

    fn peek_char(&self) -> char {
        if self.location.position < self.source.len() {
            self.source[self.location.position]
        } else {
            '\0'
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{Lexer, SourceLocation, SourceSpan, Token, TokenData, TokenKind};

    #[test]
    fn empty_file() {
        let filepath = "empty.langite";
        let source = "";
        let mut lexer = Lexer::new(filepath.to_string(), source);
        assert_eq!(
            lexer.next_token(),
            Ok(Token {
                kind: TokenKind::EndOfFile,
                location: SourceSpan {
                    filepath: filepath.to_string(),
                    start: SourceLocation {
                        position: 0,
                        line: 1,
                        column: 1
                    },
                    end: SourceLocation {
                        position: 0,
                        line: 1,
                        column: 1
                    },
                },
                data: TokenData::None
            })
        );
    }

    #[test]
    fn single_char_tokens() {
        let filepath = "single_char_tokens.langite";
        let source = "+ - * /";
        let mut lexer = Lexer::new(filepath.to_string(), source);
        assert_eq!(
            lexer.next_token(),
            Ok(Token {
                kind: TokenKind::Plus,
                location: SourceSpan {
                    filepath: filepath.to_string(),
                    start: SourceLocation {
                        position: 0,
                        line: 1,
                        column: 1
                    },
                    end: SourceLocation {
                        position: 1,
                        line: 1,
                        column: 2
                    }
                },
                data: TokenData::None,
            })
        );
        assert_eq!(
            lexer.next_token(),
            Ok(Token {
                kind: TokenKind::Minus,
                location: SourceSpan {
                    filepath: filepath.to_string(),
                    start: SourceLocation {
                        position: 2,
                        line: 1,
                        column: 3
                    },
                    end: SourceLocation {
                        position: 3,
                        line: 1,
                        column: 4
                    }
                },
                data: TokenData::None,
            })
        );
        assert_eq!(
            lexer.next_token(),
            Ok(Token {
                kind: TokenKind::Asterisk,
                location: SourceSpan {
                    filepath: filepath.to_string(),
                    start: SourceLocation {
                        position: 4,
                        line: 1,
                        column: 5
                    },
                    end: SourceLocation {
                        position: 5,
                        line: 1,
                        column: 6
                    }
                },
                data: TokenData::None,
            })
        );
        assert_eq!(
            lexer.next_token(),
            Ok(Token {
                kind: TokenKind::Slash,
                location: SourceSpan {
                    filepath: filepath.to_string(),
                    start: SourceLocation {
                        position: 6,
                        line: 1,
                        column: 7
                    },
                    end: SourceLocation {
                        position: 7,
                        line: 1,
                        column: 8
                    }
                },
                data: TokenData::None,
            })
        );
        assert_eq!(
            lexer.next_token(),
            Ok(Token {
                kind: TokenKind::EndOfFile,
                location: SourceSpan {
                    filepath: filepath.to_string(),
                    start: SourceLocation {
                        position: 7,
                        line: 1,
                        column: 8
                    },
                    end: SourceLocation {
                        position: 7,
                        line: 1,
                        column: 8
                    }
                },
                data: TokenData::None,
            })
        );
    }
}
