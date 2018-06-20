use ::ast::*;

#[derive(Debug, Eq, PartialEq)]
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
