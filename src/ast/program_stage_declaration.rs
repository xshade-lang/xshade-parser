use ::ast::*;

#[derive(Debug, Eq, PartialEq)]
pub struct ProgramStageDeclaration {
    pub span: Span,
    pub stage_name: Identifier,
    pub function: FunctionDeclaration,
}

impl Spanned for ProgramStageDeclaration {
    fn get_span(&self) -> Span {
        self.span
    }
}
