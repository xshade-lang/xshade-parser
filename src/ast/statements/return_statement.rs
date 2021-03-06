use ::ast::*;

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ReturnStatement {
    pub span: Span,
    pub expression: Expression,
}

impl Spanned for ReturnStatement {
    fn get_span(&self) -> Span {
        self.span
    }
}
