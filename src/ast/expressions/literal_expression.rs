use ::ast::*;

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct LiteralExpression {
    pub span: Span,
    pub value: String,
    pub literal_expression_type: LiteralType,
}

impl Spanned for LiteralExpression {
    fn get_span(&self) -> Span {
        self.span
    }
}
