use ::ast::*;

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct IndexAccesorExpression {
    pub span: Span,
    pub index_expression: Box<Expression>,
    pub indexee_expression: Box<Expression>,
}

impl Spanned for IndexAccesorExpression {
    fn get_span(&self) -> Span {
        self.span
    }
}

impl Execute for IndexAccesorExpression {
    fn execute(&self) -> Option<i32> {
        None
    }
}
