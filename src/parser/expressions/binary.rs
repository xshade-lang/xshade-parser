use ::parser::*;

fn fold_expressions(mut lhs: Expression, mut rhs: Vec<(OperatorType, Expression)>) -> Expression {
    while let Some(r) = rhs.pop() {
        lhs = Expression::Binary(BinaryExpression {
            span: Span::from_to(lhs.get_span(), r.1.get_span()),
            operator: r.0,
            left_hand: Box::new(lhs),
            right_hand: Box::new(r.1),
        });
    }

    lhs
}

named!(pub parse_equality<NomSpan, Expression>,
    do_parse!(
        ws0 >>
        lhs: parse_additive >>
        ws0 >>
        rhs: many0!(do_parse!(
            ws0 >>
            operator: alt!(tag!("==") | tag!(">=") | tag!("<=")) >>
            ws0 >>
            rhs: parse_additive >>
            ws0 >>
            ((parse_operator_type(&operator), rhs))
        )) >>
    (fold_expressions(lhs, rhs)))
);

named!(pub parse_additive<NomSpan, Expression>,
    do_parse!(
        ws0 >>
        lhs: parse_multiplicative >>
        ws0 >>
        rhs: many0!(do_parse!(
            ws0 >>
            operator: alt!(tag!("+") | tag!("-")) >>
            ws0 >>
            rhs: parse_multiplicative >>
            ws0 >>
            ((parse_operator_type(&operator), rhs))
        )) >>
    (fold_expressions(lhs, rhs)))
);

named!(pub parse_multiplicative<NomSpan, Expression>,
    do_parse!(
        ws0 >>
        lhs: parse_unary >>
        ws0 >>
        rhs: many0!(do_parse!(
            ws0 >>
            operator: alt!(tag!("*") | tag!("/")) >>
            ws0 >>
            rhs: parse_unary >>
            ws0 >>
            ((parse_operator_type(&operator), rhs))
        )) >>
    (fold_expressions(lhs, rhs)))
);
