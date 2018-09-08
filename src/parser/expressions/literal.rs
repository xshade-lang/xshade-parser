use ::parser::*;

named!(pub parse_literal<NomSpan, Expression>,
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
