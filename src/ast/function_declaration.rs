use ::ast::*;

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct FunctionDeclaration {
    pub span: Span,
    pub function_name: Identifier,
    pub arguments: Vec<FunctionArgumentDeclaration>,
    pub block: BlockDeclaration,
    pub return_type_name: TypeIdentifier,
}

impl Spanned for FunctionDeclaration {
    fn get_span(&self) -> Span {
        self.span
    }
}
