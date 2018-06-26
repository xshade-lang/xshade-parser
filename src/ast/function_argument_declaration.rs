use ::ast::*;

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct FunctionArgumentDeclaration {
    pub span: Span,
    pub argument_name: Identifier,
    pub argument_type_name: TypeIdentifier,
}

impl Spanned for FunctionArgumentDeclaration {
    fn get_span(&self) -> Span {
        self.span
    }
}
