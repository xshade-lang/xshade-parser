use ::std::convert::From;
use ::std::error::Error;
use ::std::fmt;
use ::ast::Span;

pub type ParseResult<T> = Result<T, ParseError>;

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub enum ParseErrorKind {
    Unknown = 0,
    UnrecognizedCharacter = 1,
    InvalidTopLevelItem = 1000,
    InvalidStructName = 2000,
    InvalidFunctionName = 3000,
    MissingArgumentList = 3001,
    MissingOpenCurlyBraces = 5000,
    MissingClosingCurlyBraces = 5001,
    MissingError = 999999,
}

impl From<u32> for ParseErrorKind {
    fn from(value: u32) -> Self {
        match value {
            1 => ParseErrorKind::UnrecognizedCharacter,
            1000 => ParseErrorKind::InvalidTopLevelItem,
            2000 => ParseErrorKind::InvalidStructName,
            3000 => ParseErrorKind::InvalidFunctionName,
            3001 => ParseErrorKind::MissingArgumentList,
            5000 => ParseErrorKind::MissingOpenCurlyBraces,
            5001 => ParseErrorKind::MissingClosingCurlyBraces,
            999999 => ParseErrorKind::MissingError,

            _ => ParseErrorKind::Unknown,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
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
            ParseErrorKind::InvalidTopLevelItem => write!(f, "ParseError: Invalid top level item at {}.", self.span),
            ParseErrorKind::MissingOpenCurlyBraces => write!(f, "ParseError: Missing opening curly braces at {}.", self.span),
            ParseErrorKind::MissingClosingCurlyBraces => write!(f, "ParseError: Missing closing curly braces at {}.", self.span),
            ParseErrorKind::InvalidStructName => write!(f, "ParseError: Invalid struct name at {}.", self.span),
            ParseErrorKind::InvalidFunctionName => write!(f, "ParseError: Invalid function name at {}.", self.span),
            ParseErrorKind::MissingArgumentList => write!(f, "ParseError: Missing argument list at {}.", self.span),
            ParseErrorKind::MissingError => write!(f, "ParseError: A custom error is missing in a nom parser."),
        }
    }
}
