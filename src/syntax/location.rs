use derive_more::Display;

#[derive(Clone, Copy, Debug, Display)]
#[display(fmt = "{line}:{column}")]
pub struct SourceLocation {
    pub position: usize,
    pub line: usize,
    pub column: usize,
}

#[derive(Clone, Copy, Debug, Display)]
#[display(fmt = "{filepath}:{start}")]
pub struct SourceSpan<'filepath> {
    pub filepath: &'filepath str,
    pub start: SourceLocation,
    pub end: SourceLocation,
}

impl SourceSpan<'static> {
    pub fn unknown() -> Self {
        Self {
            filepath: "unknown file",
            start: SourceLocation {
                position: 0,
                line: 1,
                column: 1,
            },
            end: SourceLocation {
                position: 0,
                line: 1,
                column: 1,
            },
        }
    }
}

impl<'filepath> SourceSpan<'filepath> {
    pub fn combine(a: impl GetLocation<'filepath>, b: impl GetLocation<'filepath>) -> Self {
        let (a, b) = (a.get_location(), b.get_location());
        assert_eq!(a.filepath, b.filepath);
        if a.start.position <= b.end.position {
            Self {
                filepath: a.filepath,
                start: a.start,
                end: b.end,
            }
        } else {
            Self {
                filepath: a.filepath,
                start: b.start,
                end: a.end,
            }
        }
    }
}

pub trait GetLocation<'filepath> {
    fn get_location(&self) -> SourceSpan<'filepath>;
}

impl<'filepath, T> GetLocation<'filepath> for T
where
    T: std::ops::Deref,
    T::Target: GetLocation<'filepath>,
{
    fn get_location(&self) -> SourceSpan<'filepath> {
        (**self).get_location()
    }
}

impl<'filepath> GetLocation<'filepath> for SourceSpan<'filepath> {
    fn get_location(&self) -> SourceSpan<'filepath> {
        *self
    }
}
