use ::ast::*;

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct VariableExpression {
    pub span: Span,
    pub variable_name: String,
}

impl Spanned for VariableExpression {
    fn get_span(&self) -> Span {
        self.span
    }
}

impl Execute for VariableExpression {
    fn execute(&self) -> Option<i32> {
        None
    }
}
