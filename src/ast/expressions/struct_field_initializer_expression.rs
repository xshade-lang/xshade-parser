use ::ast::*;

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct StructFieldInitializerExpression {
    pub span: Span,
    pub struct_field_name: Identifier,
    pub initializer: Box<Expression>,
}

impl Spanned for StructFieldInitializerExpression {
    fn get_span(&self) -> Span {
        self.span
    }
}

impl Execute for StructFieldInitializerExpression {
    fn execute(&self) -> Option<i32> {
        None
    }
}
