use ::ast::*;

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct IndexAccesorExpression {
    pub span: Span,
    pub variable_name: Identifier,
    pub access_expression: Box<Expression>,
}

impl Spanned for IndexAccesorExpression {
    fn get_span(&self) -> Span {
        self.span
    }
}
