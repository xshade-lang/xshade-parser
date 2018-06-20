#[macro_use]
use ::ast::*;

#[derive(Debug, Eq, PartialEq)]
pub struct FieldAccessorExpression {
    pub span: Span,
    pub variable_name: Identifier,
    pub field_name: Identifier,
}

impl Spanned for FieldAccessorExpression {
    fn get_span(&self) -> Span {
        self.span
    }
}
