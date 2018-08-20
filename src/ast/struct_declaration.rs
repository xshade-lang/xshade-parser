use ::ast::*;

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct StructDeclaration {
    pub span: Span,
    pub struct_name: Identifier,
    pub type_arguments: Vec<Identifier>,
    pub struct_member: Vec<StructMemberDeclaration>,
}

impl Spanned for StructDeclaration {
    fn get_span(&self) -> Span {
        self.span
    }
}
