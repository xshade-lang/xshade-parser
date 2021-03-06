use ::parser::*;

/*
 * Group Operators
 * `( )` group
 * Unary Operators
 * `-` negate
 * `!` binary not
 * `~` bitwise complement
 * `.foo` field access
 * `[0]` index access
 * Binary Operators
 * `*` multiply
 * `/` divide
 * `%` modulo
 * 
 * `+` add
 * `-` divide
 * 
 * `<<` shift left
 * `>>` shift right
 * 
 * `>=` greater or equal to
 * `<=` less or equal to
 * 
 * `==` equals
 * `!=` not equal
 * 
 * `&` bitwise and
 * `^` bitwise not
 * `|` bitwise or
 * 
 * `&&` binary and
 * `||` binary or
*/

mod binary;
mod literal;
mod primary;
mod unary;
mod variable;
pub use self::binary::*;
pub use self::literal::*;
pub use self::primary::*;
pub use self::unary::*;
pub use self::variable::*;

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

pub fn parse_unary_operator_type(input: &NomSpan) -> UnaryOperatorType {
    match input.fragment.as_ref() {
        "-" => UnaryOperatorType::Negate,
        "!" => UnaryOperatorType::Not,
        "~" => UnaryOperatorType::Complement,
        _ => panic!("unknown operator, this should never happen, if it happens a parser did not check correctly"),
    }
}

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
    fn it_does_negation() {
        run_and_compare("10 - -5", 15);
    }
}
