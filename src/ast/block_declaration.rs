use ::ast::*;

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct BlockDeclaration {
    pub span: Span,
    pub statements: Vec<Statement>,
}

impl Spanned for BlockDeclaration {
    fn get_span(&self) -> Span {
        self.span
    }
}
