use ::ast::*;

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct InfixExpression {
    pub span: Span,
    pub operator: OperatorType,
    pub left_hand: Box<Expression>,
    pub right_hand: Box<Expression>,
}

impl Spanned for InfixExpression {
    fn get_span(&self) -> Span {
        self.span
    }
}
