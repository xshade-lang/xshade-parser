use ::ast::*;

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct NegationExpression {
    pub span: Span,
    pub expression: Box<Expression>,
}

impl Spanned for NegationExpression {
    fn get_span(&self) -> Span {
        self.span
    }
}

impl Execute for NegationExpression {
    fn execute(&self) -> Option<i32> {
        match self.expression.execute() {
            Some(i) => Some(-i),
            None => None,
        }
    }
}
