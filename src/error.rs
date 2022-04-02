use derive_more::IsVariant;

use crate::{
    source_location::SourceLocation,
    token::{Token, TokenKind},
};

#[derive(Debug, Clone, PartialEq, IsVariant)]
pub enum LexerError {
    InvalidChar {
        invalid_char_location: SourceLocation,
        invalid_char: char,
    },
    UnclosedBlockComment {
        open_comment_location: SourceLocation,
    },
    DigitTooBigForBase {
        number_location: SourceLocation,
        invalid_digit_location: SourceLocation,
        invalid_digit_char: char,
        invalid_digit: u128,
    },
    UnclosedStringLiteral {
        open_quote_location: SourceLocation,
    },
    InvalidEscapeChar {
        string_location: SourceLocation,
        invalid_escape_char_location: SourceLocation,
        invlaid_escape_char: char,
    },
}

#[derive(Debug, Clone, PartialEq, IsVariant)]
pub enum ParsingError {
    LexerError(LexerError),
    ExpectedExpression { got: Token },
    ExpectedToken { expected: TokenKind, got: Token },
    ExpectedCommaOrNewline { got: Token },
    ExpectedNameOrWildcard { got: Token },
}

impl From<LexerError> for ParsingError {
    fn from(error: LexerError) -> Self {
        ParsingError::LexerError(error)
    }
}
