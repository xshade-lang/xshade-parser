use ::parser::*;

/*
 * Group Operators
 * `( )`
 * Unary Operators
 * `-`
 * `!`
 * `~`
 * `.foo`
 * `[0]`
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

mod binary;
mod literal;
mod variable;
use self::binary::*;
use self::literal::*;
use self::variable::*;

named!(pub parse_expression<NomSpan, Expression>,
    do_parse!(
        expression: parse_equality >>
    (expression))
);

pub fn parse_operator_type(input: &NomSpan) -> OperatorType {
    match input.fragment.as_ref() {
        "==" => OperatorType::Equality,
        "+" => OperatorType::Plus,
        "-" => OperatorType::Minus,
        "*" => OperatorType::Multiply,
        "/" => OperatorType::Divide,
        _ => panic!("unknown operator, this should never happen, if it happens a parser did not check correctly"),
    }
}

fn fold_unary(mut primary: Expression, mut unary: Vec<UnaryExpressionData>) -> Expression {
    unary.reverse();
    while let Some(unary_expression_data) = unary.pop() {
        primary = match unary_expression_data {
            UnaryExpressionData::Field(span, identifier) => Expression::FieldAccessor(FieldAccessorExpression {
                span: span,
                accessee_expression: Box::new(primary),
                field_name: identifier,
            }),
            UnaryExpressionData::Index(span, expression) => Expression::IndexAccessor(IndexAccesorExpression {
                span: span,
                indexee_expression: Box::new(primary),
                index_expression: Box::new(expression),
            }),
        }
    }

    primary
}

#[derive(Debug)]
enum UnaryExpressionData {
    Index(Span, Expression),
    Field(Span, String),
}

named!(pub parse_unary<NomSpan, Expression>,
    alt!(
        parse_negated_unary |
        parse_non_negated_unary
    )
);

named!(pub parse_non_negated_unary<NomSpan, Expression>,
    alt!(
        do_parse!(
            primary: parse_primary >>
            unary: many1!(alt!(
                parse_indexing |
                parse_field
            )) >>
        (fold_unary(primary, unary))) |
        parse_primary
    )
);

named!(parse_negated_unary<NomSpan, Expression>,
    do_parse!(
        ws0 >>
        sign: tag!("-") >>
        ws0 >>
        expression: parse_non_negated_unary >>
        ws0 >>
    (Expression::Negation(NegationExpression {
        span: Span::from_to(Span::from_nom_span(&sign), expression.get_span()),
        expression: Box::new(expression),
    })))
);

named!(parse_primary<NomSpan, Expression>,
    alt!(
        parse_grouped |
        parse_literal |
        parse_variable
    )
);

named!(parse_field<NomSpan, UnaryExpressionData>,
    do_parse!(
        ws0 >>
        dot: tag!(".") >>
        accessor: recognize!(
            do_parse!(
                one_of!("_abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ") >>
                many0!(one_of!("_abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789")) >>
                ()
            )
        ) >>
        ws0 >>
    (UnaryExpressionData::Field(Span::from_to(Span::from_nom_span(&dot), Span::from_nom_span(&accessor)), accessor.fragment.to_string())))
);

named!(parse_indexing<NomSpan, UnaryExpressionData>,
    do_parse!(
        ws0 >>
        open_tag: tag!("[") >>
        ws0 >>
        indexer: parse_expression >>
        ws0 >>
        close_tag: tag!("]") >>
    (UnaryExpressionData::Index(Span::from_to(Span::from_nom_span(&open_tag), Span::from_nom_span(&close_tag)), indexer)))
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

    #[test]
    fn it_works() {
        let result = parse_expression(NomSpan::new(CompleteStr("-b[a].c"))).unwrap();
        assert!(result.0.fragment.len() == 0);
    }
}
