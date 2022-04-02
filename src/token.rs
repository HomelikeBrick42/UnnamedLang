use derive_more::{Display, IsVariant, Unwrap};

use crate::source_location::SourceLocation;

#[derive(Debug, Clone, PartialEq, Display, IsVariant, Unwrap)]
pub enum TokenKind {
    #[display(fmt = "EOF")]
    EndOfFile,
    #[display(fmt = "newline")]
    Newline,
    #[display(fmt = "'{}'", _0)]
    Name(String),
    #[display(fmt = "{:?}", _0)]
    String(String),
    #[display(fmt = "{}", _0)]
    Integer(u128),
    #[display(fmt = "{:.}", _0)]
    Float(f64),
    #[display(fmt = "wildcard")]
    Wildcard,
    #[display(fmt = "const")]
    Const,
    #[display(fmt = "func")]
    Func,
    #[display(fmt = "proc")]
    Proc,
    #[display(fmt = "return")]
    Return,
    #[display(fmt = "if")]
    If,
    #[display(fmt = "else")]
    Else,
    #[display(fmt = "(")]
    OpenParenthesis,
    #[display(fmt = ")")]
    CloseParenthesis,
    #[display(fmt = "{{")]
    OpenBrace,
    #[display(fmt = "}}")]
    CloseBrace,
    #[display(fmt = "[")]
    OpenSquareBracket,
    #[display(fmt = "]")]
    CloseSquareBracket,
    #[display(fmt = ":")]
    Colon,
    #[display(fmt = ",")]
    Comma,
    #[display(fmt = "=")]
    Equals,
    #[display(fmt = "<-")]
    LeftArrow,
    #[display(fmt = "->")]
    RightArrow,
    #[display(fmt = "+")]
    Plus,
    #[display(fmt = "-")]
    Minus,
    #[display(fmt = "*")]
    Asterisk,
    #[display(fmt = "/")]
    Slash,
    #[display(fmt = "%")]
    Percent,
}

impl TryFrom<char> for TokenKind {
    type Error = ();

    fn try_from(chr: char) -> Result<TokenKind, ()> {
        Ok(match chr {
            '\0' => TokenKind::EndOfFile,
            '\n' => TokenKind::Newline,
            '(' => TokenKind::OpenParenthesis,
            ')' => TokenKind::CloseParenthesis,
            '{' => TokenKind::OpenBrace,
            '}' => TokenKind::CloseBrace,
            '[' => TokenKind::OpenSquareBracket,
            ']' => TokenKind::CloseSquareBracket,
            ':' => TokenKind::Colon,
            ',' => TokenKind::Comma,
            '=' => TokenKind::Equals,
            '+' => TokenKind::Plus,
            '-' => TokenKind::Minus,
            '*' => TokenKind::Asterisk,
            '/' => TokenKind::Slash,
            '%' => TokenKind::Percent,
            _ => return Err(()),
        })
    }
}

impl TryFrom<(char, char)> for TokenKind {
    type Error = ();

    fn try_from(chr: (char, char)) -> Result<TokenKind, ()> {
        Ok(match chr {
            ('<', '-') => TokenKind::LeftArrow,
            ('-', '>') => TokenKind::RightArrow,
            _ => return Err(()),
        })
    }
}

impl From<String> for TokenKind {
    fn from(name: String) -> TokenKind {
        let mut iter = name.chars();
        if let Some(first) = iter.next() {
            if let Some(second) = iter.next() {
                if let Ok(kind) = TokenKind::try_from((first, second)) {
                    return kind;
                }
            }
            if let Ok(kind) = TokenKind::try_from(first) {
                return kind;
            }
        }

        match &name as &str {
            "_" => TokenKind::Wildcard,
            "const" => TokenKind::Const,
            "func" => TokenKind::Func,
            "proc" => TokenKind::Proc,
            "return" => TokenKind::Return,
            "if" => TokenKind::If,
            "else" => TokenKind::Else,
            _ => TokenKind::Name(name),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Display)]
#[display(fmt = "{}", kind)]
pub struct Token {
    pub kind: TokenKind,
    pub location: SourceLocation,
    pub length: usize,
}
