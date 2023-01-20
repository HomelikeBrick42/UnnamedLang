use derive_more::Display;

use super::{GetLocation, SourceSpan};

#[derive(Clone, Debug, Display, PartialEq, Eq)]
pub enum TokenKind<'source> {
    #[display(fmt = "{{newline}}")]
    Newline,
    #[display(fmt = "'{_0}'")]
    Name(&'source str),
    #[display(fmt = "{_0}")]
    Integer(u128),
    #[display(fmt = "{_0:?}")]
    String(String),
    #[display(fmt = "(")]
    OpenParenthesis,
    #[display(fmt = ")")]
    CloseParenthesis,
    #[display(fmt = "{{")]
    OpenBrace,
    #[display(fmt = "}}")]
    CloseBrace,
    #[display(fmt = ",")]
    Comma,
    #[display(fmt = ":")]
    Colon,
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
    #[display(fmt = "proc")]
    ProcKeyword,
    #[display(fmt = "return")]
    ReturnKeyword,
    #[display(fmt = "do")]
    DoKeyword,
    #[display(fmt = "let")]
    LetKeyword,
    #[display(fmt = "var")]
    VarKeyword,
    #[display(fmt = "const")]
    ConstKeyword,
}

#[derive(Clone, Debug, Display)]
#[display(fmt = "{kind}")]
pub struct Token<'filepath, 'source> {
    pub kind: TokenKind<'source>,
    pub span: SourceSpan<'filepath>,
}

impl<'filepath, 'source> GetLocation<'filepath> for Token<'filepath, 'source> {
    fn get_location(&self) -> SourceSpan<'filepath> {
        self.span
    }
}
