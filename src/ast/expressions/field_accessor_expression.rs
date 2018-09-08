use ::ast::*;

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct FieldAccessorExpression {
    pub span: Span,
    pub accessee_expression: Box<Expression>,
    pub field_name: String,
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
