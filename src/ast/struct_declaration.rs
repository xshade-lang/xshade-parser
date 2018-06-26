use ::ast::*;

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct StructDeclaration {
    pub span: Span,
    pub struct_name: Identifier,
    pub struct_member: Vec<StructMemberDeclaration>,
}

impl Spanned for StructDeclaration {
    fn get_span(&self) -> Span {
        self.span
    }
}
