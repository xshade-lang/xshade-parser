use ::ast::*;

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct StructMemberDeclaration {
    pub span: Span,
    pub struct_member_name: Identifier,
    pub struct_member_type_name: TypeIdentifier,
}

impl Spanned for StructMemberDeclaration {
    fn get_span(&self) -> Span {
        self.span
    }
}
