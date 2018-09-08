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

named!(pub parse_unary<NomSpan, Expression>,
    alt!(
        parse_index_accessor |
        parse_unary_2
    )
);

named!(pub parse_unary_2<NomSpan, Expression>,
    alt!(
        parse_field_accessor |
        parse_unary_3
    )
);

named!(pub parse_unary_3<NomSpan, Expression>,
    alt!(
        parse_grouped |
        parse_literal |
        parse_variable |
        parse_negated
    )
);

named!(parse_unary_3_non_negated<NomSpan, Expression>,
    alt!(
        parse_primary
    )
);

named!(parse_primary<NomSpan, Expression>,
    alt!(
        parse_grouped |
        parse_literal |
        parse_variable
    )
);

fn fold_field_accessors(mut accessee: Expression, mut accessors: Vec<(Span, String)>) -> Expression {
    accessors.reverse();
    while let Some((span, accessor)) = accessors.pop() {
        accessee = Expression::FieldAccessor(FieldAccessorExpression {
            span: Span::from_to(accessee.get_span(), span),
            accessee_expression: Box::new(accessee),
            field_name: accessor,
        });
    }

    accessee
}

named!(parse_field_accessor<NomSpan, Expression>,
    do_parse!(
        ws0 >>
        accessee: parse_unary_3 >>
        ws0 >>
        accessors: many1!(
            do_parse!(
                ws0 >>
                tag!(".") >>
                ws0 >>
                accessor: recognize!(
                    do_parse!(
                        one_of!("_abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ") >>
                        many0!(one_of!("_abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789")) >>
                        ()
                    )
                ) >>
                ws0 >>
            ((Span::from_nom_span(&accessor), accessor.fragment.to_string())))
        ) >>
        ws0 >>
    (fold_field_accessors(accessee, accessors)))
);

fn fold_index_accessors(mut indexee: Expression, mut indexers: Vec<(Span, Expression)>) -> Expression {
    indexers.reverse();
    while let Some((span, indexer)) = indexers.pop() {
        indexee = Expression::IndexAccessor(IndexAccesorExpression {
            span: Span::from_to(indexee.get_span(), span),
            indexee_expression: Box::new(indexee),
            index_expression: Box::new(indexer),
        });
    }

    indexee
}

named!(parse_index_accessor<NomSpan, Expression>,
    do_parse!(
        ws0 >>
        indexee: parse_unary_2 >>
        ws0 >>
        indexers: many1!(
            do_parse!(
                ws0 >>
                tag!("[") >>
                ws0 >>
                indexer: parse_expression >>
                ws0 >>
                close_tag: tag!("]") >>
            ((Span::from_nom_span(&close_tag), indexer)))
        ) >>
        ws0 >>
    (fold_index_accessors(indexee, indexers)))
);

named!(parse_negated<NomSpan, Expression>,
    do_parse!(
        ws0 >>
        sign: tag!("-") >>
        ws0 >>
        expression: parse_unary_3_non_negated >>
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

    #[test]
    fn it_works() {
        let result = parse_expression(NomSpan::new(CompleteStr("b.a[5].c[5][6]"))).unwrap();
        println!("{:#?}", result);
        panic!()
    }
}
