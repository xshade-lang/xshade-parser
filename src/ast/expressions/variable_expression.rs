use ::ast::*;

#[derive(Debug, Eq, PartialEq)]
pub struct VariableExpression {
    pub span: Span,
    pub variable_name: Identifier,
}

impl Spanned for VariableExpression {
    fn get_span(&self) -> Span {
        self.span
    }
}
