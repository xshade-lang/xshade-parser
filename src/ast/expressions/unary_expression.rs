use ::ast::*;

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct UnaryExpression {
    pub span: Span,
    pub operator: UnaryOperatorType,
    pub expression: Box<Expression>,
}

impl Spanned for UnaryExpression {
    fn get_span(&self) -> Span {
        self.span
    }
}

impl Execute for UnaryExpression {
    fn execute(&self) -> Option<i32> {
        let value = self.expression.execute()?;

        match self.operator {
            UnaryOperatorType::Negate => Some(-value),
            _ => None,
        }
    }
}
