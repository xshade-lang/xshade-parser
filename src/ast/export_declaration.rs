use ::ast::*;

#[derive(Debug, Eq, PartialEq)]
pub enum ExportItem {
    Named(Identifier),
    All
}

#[derive(Debug, Eq, PartialEq)]
pub struct ExportDeclaration {
    pub span: Span,
    pub items: Vec<ExportItem>,
}

impl Spanned for ExportDeclaration {
    fn get_span(&self) -> Span {
        self.span
    }
}
