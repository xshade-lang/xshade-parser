use ::ast::*;

#[derive(Debug, Eq, PartialEq)]
pub struct LocalStatement {
    pub span: Span,
    pub symbol_name: Identifier,
    pub expression: Expression,
}

impl Spanned for LocalStatement {
    fn get_span(&self) -> Span {
        self.span
    }
}
