use ::std::convert::From;
use ::std::error::Error;
use ::std::fmt;
use ::ast::Span;

pub type ParseResult<T> = Result<T, ParseError>;

#[derive(Debug, Copy, Clone)]
pub enum ParseErrorKind {
    Unknown = 0,
    UnrecognizedCharacter = 1,
    MissingOpenCurlyBraces = 1000,
    MissingClosingCurlyBraces = 1001,
    InvalidStructName = 2000,
}

impl From<u32> for ParseErrorKind {
  fn from(value: u32) -> Self {
    match value {
        1000 => ParseErrorKind::MissingOpenCurlyBraces,
        1001 => ParseErrorKind::MissingClosingCurlyBraces,
        2000 => ParseErrorKind::InvalidStructName,
        _ => ParseErrorKind::Unknown,
    }
  }
}

#[derive(Debug)]
pub struct ParseError {
    span: Span,
    kind: ParseErrorKind,
}

impl ParseError {
    pub fn new(span: Span, kind: ParseErrorKind) -> ParseError {
        ParseError {
            span,
            kind,
        }
    }
}

impl Error for ParseError {
    fn description(&self) -> &str {
        "ParseError"
    }

    fn cause(&self) -> Option<&Error> {
        None
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.kind {
            ParseErrorKind::Unknown => write!(f, "ParseError: Unknown error at {}.", self.span),
            ParseErrorKind::UnrecognizedCharacter => write!(f, "ParseError: Unrecognized character at {}.", self.span),
            ParseErrorKind::MissingOpenCurlyBraces => write!(f, "ParseError: Missing opening curly braces at {}.", self.span),
            ParseErrorKind::MissingClosingCurlyBraces => write!(f, "ParseError: Missing closing curly braces at {}.", self.span),
            ParseErrorKind::InvalidStructName => write!(f, "ParseError: Invalid struct name at {}.", self.span),
        }
    }
}
