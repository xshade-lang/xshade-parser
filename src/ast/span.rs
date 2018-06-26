use ::std::fmt;
use nom::types::CompleteStr;
use nom_locate::LocatedSpan;

type NomSpan<'a> = LocatedSpan<CompleteStr<'a>>;

#[derive(Eq, PartialEq, Copy, Clone, Serialize, Deserialize)]
pub struct Span {
    pub offset: usize,
    pub length: usize,
    pub line: usize,
    pub column: usize,
}

impl Span {
    pub fn new(offset: usize, length: usize, line: usize, column: usize) -> Span {
        Span {
            offset: offset,
            length: length,
            line: line,
            column: column,
        }
    }

    pub fn from_nom_span(span: &NomSpan) -> Span {
        Span {
            offset: span.offset,
            length: span.fragment.len(),
            line: span.line as usize,
            column: span.get_column(),
        }
    }

    pub fn from_to(from: Span, to: Span) -> Span {
        Span {
            offset: from.offset,
            length: to.offset - from.offset + to.length,
            line: from.line,
            column: from.column,
        }
    }
}

impl fmt::Debug for Span {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Offset {} Lenght {} Line {} Column {}", self.offset, self.length, self.line, self.column)
    }
}

impl fmt::Display for Span {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Offset {} Lenght {} Line {} Column {}", self.offset, self.length, self.line, self.column)
    }
}
