use ::ast::*;

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
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

impl Execute for FieldAccessorExpression {
    fn execute(&self) -> Option<i32> {
        None
    }
}
