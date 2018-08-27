use ::nom::IResult;
use ::parser::*;

/*
 * Group Operators
 * `( )`
 * Unary Operators
 * `-`
 * `!`
 * `~`
 * Binary Operators
 * `*`
 * `/`
 * `%`
 * 
 * `+`
 * `-`
 * 
 * `<<`
 * `>>`
 * 
 * `>=`
 * `<=`
 * 
 * `==`
 * `!=`
 * 
 * `&`
 * `^`
 * `|`
 * 
 * `&&`
 * `||`
*/

named!(pub parse_expression<NomSpan, Expression>,
    do_parse!(
        expression: parse_equality >>
    (expression))
);

fn parse_operator_type(input: &NomSpan) -> OperatorType {
    match input.fragment.as_ref() {
        "==" => OperatorType::Equality,
        "+" => OperatorType::Plus,
        "-" => OperatorType::Minus,
        "*" => OperatorType::Multiply,
        "/" => OperatorType::Divide,
        _ => panic!("unknown operator, this should never happen, if it happens a parser did not check correctly"),
    }
}

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

named!(parse_equality<NomSpan, Expression>,
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

named!(parse_additive<NomSpan, Expression>,
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

named!(parse_multiplicative<NomSpan, Expression>,
    do_parse!(
        ws0 >>
        lhs: parse_primary >>
        ws0 >>
        rhs: many0!(do_parse!(
            ws0 >>
            operator: alt!(tag!("*") | tag!("/")) >>
            ws0 >>
            rhs: parse_primary >>
            ws0 >>
            ((parse_operator_type(&operator), rhs))
        )) >>
    (fold_expressions(lhs, rhs)))
);

named!(parse_primary<NomSpan, Expression>,
    alt!(
        parse_grouped |
        parse_literal |
        parse_variable |
        parse_negated
    )
);

named!(parse_primary_non_negated<NomSpan, Expression>,
    alt!(
        parse_grouped |
        parse_literal |
        parse_variable
    )
);

named!(parse_variable<NomSpan, Expression>,
    do_parse!(
        ws0 >>
        variable: recognize!(
            do_parse!(
                one_of!("_abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ") >>
                many0!(one_of!("_abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789")) >>
                ()
            )
        ) >>
        ws0 >>
    (Expression::Variable(VariableExpression {
        span: Span::from_nom_span(&variable),
        variable_name: variable.fragment.to_string(),
    })))
);

named!(parse_literal<NomSpan, Expression>,
    alt!(
        parse_int_literal
    )
);

named!(parse_int_literal<NomSpan, Expression>,
    do_parse!(
        ws0 >>
        literal: recognize!(many1!(one_of!("1234567890"))) >>
        ws0 >>
    (Expression::Literal(LiteralExpression {
        span: Span::from_nom_span(&literal),
        value: literal.fragment.to_string(),
        literal_expression_type: LiteralType::Int,
    })))
);

named!(parse_negated<NomSpan, Expression>,
    do_parse!(
        ws0 >>
        sign: tag!("-") >>
        ws0 >>
        expression: parse_primary_non_negated >>
        ws0 >>
    (Expression::Negation(NegationExpression {
        span: Span::from_to(Span::from_nom_span(&sign), expression.get_span()),
        expression: Box::new(expression),
    })))
);

named!(parse_grouped<NomSpan, Expression>,
    do_parse!(
        ws0 >>
        tag!("(") >>
        ws0 >>
        expression: parse_expression >>
        ws0 >>
        tag!(")") >>
        ws0 >>
    (expression))
);

#[cfg(test)]
mod tests {
    use super::*;

    fn run_and_compare(input: &str, expected: i32) {
        let result = parse_expression(NomSpan::new(CompleteStr(input))).unwrap();
        let result = result.1.execute().unwrap();
        assert_eq!(result, expected, "{}", input);
    }

    #[test]
    fn it_adds() {
        run_and_compare("1337 + 1", 1338);
    }

    #[test]
    fn it_subtracts() {
        run_and_compare("1337 - 1", 1336);
    }

    #[test]
    fn it_multiplies() {
        run_and_compare("11 * 3", 33);
    }

    #[test]
    fn it_divides() {
        run_and_compare("30 / 3", 10);
    }

    #[test]
    fn it_has_precedence() {
        run_and_compare("10 - 5 * 3", -5);
    }

    #[test]
    fn it_has_precedence_2() {
        run_and_compare("(10 - 5) * 3", 15);
    }
}
