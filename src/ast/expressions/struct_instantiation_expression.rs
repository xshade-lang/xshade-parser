use ::ast::*;

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
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

impl Execute for StructInstantiationExpression {
    fn execute(&self) -> Option<i32> {
        None
    }
}
