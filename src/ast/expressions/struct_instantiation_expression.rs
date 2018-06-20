use ::ast::*;

#[derive(Debug, Eq, PartialEq)]
pub struct StructInstantiationExpression {
    pub span: Span,
    pub struct_type_name: TypeIdentifier,
    pub struct_field_initializer: Vec<StructFieldInitializerExpression>,
}

impl Spanned for StructInstantiationExpression {
    fn get_span(&self) -> Span {
        self.span
    }
}
