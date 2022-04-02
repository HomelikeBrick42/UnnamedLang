use derive_more::Display;

#[derive(Debug, Clone, PartialEq, Display)]
#[display(fmt = "{}:{}:{}", filepath, line, column)]
pub struct SourceLocation {
    pub filepath: String,
    pub position: usize,
    pub line: usize,
    pub column: usize,
}
