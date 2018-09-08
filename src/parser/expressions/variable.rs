use ::parser::*;

named!(pub parse_variable<NomSpan, Expression>,
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
