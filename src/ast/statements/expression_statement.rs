use ::ast::*;

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ExpressionStatement {
    pub span: Span,
    pub expression: Expression,
}

impl Spanned for ExpressionStatement {
    fn get_span(&self) -> Span {
        self.span
    }
}
