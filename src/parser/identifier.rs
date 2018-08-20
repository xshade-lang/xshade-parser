use ::parser::*;

named!(pub parse_identifier(NomSpan) -> Identifier,
    do_parse!(
        span: recognize!(
            do_parse!(
                one_of!("_abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ") >>
                many0!(one_of!("_abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789")) >>
                ()
            )
        ) >>
        (Identifier::from_nom_span(&span))
    )
);
