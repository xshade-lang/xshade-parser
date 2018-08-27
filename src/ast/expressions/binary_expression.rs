use ::ast::*;

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct BinaryExpression {
    pub span: Span,
    pub operator: OperatorType,
    pub left_hand: Box<Expression>,
    pub right_hand: Box<Expression>,
}

impl Spanned for BinaryExpression {
    fn get_span(&self) -> Span {
        self.span
    }
}

impl Execute for BinaryExpression {
    fn execute(&self) -> Option<i32> {
        let left = self.left_hand.execute()?;
        let right = self.right_hand.execute()?;

        match self.operator {
            OperatorType::Plus => Some(left + right),
            OperatorType::Minus => Some(left - right),
            OperatorType::Multiply => Some(left * right),
            OperatorType::Divide => Some(left / right),
            _ => None,
        }
    }
}
