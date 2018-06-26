use ::ast::*;

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum ImportItem {
    Named(Identifier),
    All
}

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ImportDeclaration {
    pub span: Span,
    pub items: Vec<ImportItem>,
    pub module_id: String,
}

impl Spanned for ImportDeclaration {
    fn get_span(&self) -> Span {
        self.span
    }
}
