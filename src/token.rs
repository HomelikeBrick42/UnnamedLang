use derive_more::Display;
use enum_as_inner::EnumAsInner;

use crate::SourceSpan;

#[derive(Clone, PartialEq, Debug, Display, EnumAsInner)]
pub enum TokenKind {
    #[display(fmt = "{{EOF}}")]
    EndOfFile,
    #[display(fmt = "{{newline}}")]
    Newline,
    #[display(fmt = "{{name}}")]
    Name,
    #[display(fmt = "{{string literal}}")]
    String,
    #[display(fmt = "{{integer}}")]
    Integer,
    #[display(fmt = "(")]
    OpenParenthesis,
    #[display(fmt = ")")]
    CloseParenthesis,
    #[display(fmt = "{{")]
    OpenBrace,
    #[display(fmt = "}}")]
    CloseBrace,
    #[display(fmt = ":")]
    Colon,
    #[display(fmt = ",")]
    Comma,
    #[display(fmt = "^")]
    Caret,
    #[display(fmt = "&")]
    Ampersand,
    #[display(fmt = "=")]
    Equal,
    #[display(fmt = "<-")]
    LeftArrow,
    #[display(fmt = "->")]
    RightArrow,
    #[display(fmt = "=>")]
    FatRightArrow,
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
    #[display(fmt = "==")]
    EqualEqual,
    #[display(fmt = "!=")]
    ExclamationMarkEqual,
    #[display(fmt = "<")]
    LessThan,
    #[display(fmt = ">")]
    GreaterThan,
    #[display(fmt = "<=")]
    LessThanEqual,
    #[display(fmt = ">=")]
    GreaterThanEqual,
    #[display(fmt = "#extern")]
    ExternDirective,
    #[display(fmt = "proc")]
    ProcKeyword,
    #[display(fmt = "return")]
    ReturnKeyword,
    #[display(fmt = "let")]
    LetKeyword,
    #[display(fmt = "var")]
    VarKeyword,
    #[display(fmt = "if")]
    IfKeyword,
    #[display(fmt = "else")]
    ElseKeyword,
    #[display(fmt = "while")]
    WhileKeyword,
    #[display(fmt = "cast")]
    CastKeyword,
}

#[derive(Clone, PartialEq, Debug, Display, EnumAsInner)]
pub enum TokenData {
    #[display(fmt = "None")]
    None,
    #[display(fmt = "{:?}", _0)]
    String(String),
    #[display(fmt = "{}", _0)]
    Integer(u128),
}

#[derive(Clone, PartialEq, Debug)]
pub struct Token {
    pub kind: TokenKind,
    pub location: SourceSpan,
    pub data: TokenData,
}
