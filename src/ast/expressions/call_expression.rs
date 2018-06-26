use ::ast::*;

#[derive(Debug, Eq, PartialEq)]
pub struct CallExpression {
    pub span: Span,
    pub function_name: Identifier,
    pub arguments: Vec<Expression>,
}

impl Spanned for CallExpression {
    fn get_span(&self) -> Span {
        self.span
    }
}
