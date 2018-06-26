use nom::types::CompleteStr;
use nom_locate::LocatedSpan;

type NomSpan<'a> = LocatedSpan<CompleteStr<'a>>;

named!(pub ws0<NomSpan, NomSpan>, recognize!(many0!(one_of!(" \r\n\t"))));
named!(pub ws1<NomSpan, NomSpan>, recognize!(many1!(one_of!(" \r\n\t"))));

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_parses_zero_ws() {
        let input = NomSpan::new(CompleteStr("a"));
        let output = ws0(input).unwrap();
        assert_eq!(output.1.fragment, CompleteStr(""));
    }

    #[test]
    fn it_parses_at_least_one_ws() {
        let input = NomSpan::new(CompleteStr("  \r\n  a"));
        let output = ws0(input).unwrap();
        assert_eq!(output.1.fragment, CompleteStr("  \r\n  "));
    }

    #[test]
    fn it_fails_on_zero_ws() {
        let input = NomSpan::new(CompleteStr("a"));
        let output = ws1(input).is_err();
        assert!(output);
    }

    #[test]
    fn it_parses_one_or_more_ws() {
        let input = NomSpan::new(CompleteStr("  \r\n  a"));
        let output = ws0(input).unwrap();
        assert_eq!(output.1.fragment, CompleteStr("  \r\n  "));
    }
}
