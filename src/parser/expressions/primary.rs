use ::parser::*;

named!(pub parse_primary<NomSpan, Expression>,
    alt!(
        parse_grouped |
        parse_literal |
        parse_variable
    )
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
