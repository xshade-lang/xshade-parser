use ::ast::*;

#[derive(Debug, Eq, PartialEq)]
pub struct ProgramDeclaration {
    pub span: Span,
    pub program_name: Identifier,
    pub program_stages: Vec<ProgramStageDeclaration>,
}

impl Spanned for ProgramDeclaration {
    fn get_span(&self) -> Span {
        self.span
    }
}
