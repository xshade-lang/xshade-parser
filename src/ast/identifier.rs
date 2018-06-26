use nom::types::CompleteStr;
use nom_locate::LocatedSpan;
use ast::*;

type NomSpan<'a> = LocatedSpan<CompleteStr<'a>>;

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Identifier {
    pub span: Span,
    pub name: String,
}

impl Identifier {
    pub fn new(name: &str, span: Span) -> Identifier {
        Identifier {
            span: span,
            name: name.to_string(),
        }
    }

    pub fn from_nom_span(span: &NomSpan) -> Identifier {
        Identifier {
            span: Span::new(span.offset, span.fragment.len(), span.line as usize, span.get_column() as usize),
            name: span.fragment.to_string(),
        }
    }
}

impl Spanned for Identifier {
    fn get_span(&self) -> Span {
        self.span
    }
}

pub type TypeIdentifier = Identifier;
