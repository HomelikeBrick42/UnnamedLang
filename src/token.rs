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
    #[display(fmt = "=")]
    Equal,
    #[display(fmt = "+")]
    Plus,
    #[display(fmt = "-")]
    Minus,
    #[display(fmt = "*")]
    Asterisk,
    #[display(fmt = "/")]
    Slash,
    #[display(fmt = "<")]
    LessThan,
    #[display(fmt = ">")]
    GreaterThan,
    #[display(fmt = "<=")]
    LessThanEqual,
    #[display(fmt = ">=")]
    GreaterThanEqual,
    #[display(fmt = "#compiler")]
    CompilerDirective,
    #[display(fmt = "proc")]
    ProcKeyword,
    #[display(fmt = "let")]
    LetKeyword,
    #[display(fmt = "var")]
    VarKeyword,
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
