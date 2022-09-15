use derive_more::Display;

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Display)]
#[display(fmt = "{}:{}", line, column)]
pub struct SourceLocation {
    pub position: usize,
    pub line: usize,
    pub column: usize,
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Display)]
#[display(fmt = "{}:{}", filepath, start)]
pub struct SourceSpan {
    pub filepath: String,
    pub start: SourceLocation,
    pub end: SourceLocation,
}

impl SourceSpan {
    pub fn combine_spans(a: &SourceSpan, b: &SourceSpan) -> SourceSpan {
        assert_eq!(a.filepath, b.filepath);
        assert!(a.start.position <= b.end.position);
        SourceSpan {
            filepath: a.filepath.clone(),
            start: a.start.clone(),
            end: b.end.clone(),
        }
    }

    pub fn get_length(&self) -> usize {
        self.end.position - self.start.position
    }
}
