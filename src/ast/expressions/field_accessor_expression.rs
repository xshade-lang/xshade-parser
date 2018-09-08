use ::ast::*;

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct FieldAccessorExpression {
    pub span: Span,
    pub field_name: String,
    pub accessee_expression: Box<Expression>,
}

impl Spanned for FieldAccessorExpression {
    fn get_span(&self) -> Span {
        self.span
    }
}

impl Execute for FieldAccessorExpression {
    fn execute(&self) -> Option<i32> {
        None
    }
}
