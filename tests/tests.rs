extern crate xshade_parser;

use xshade_parser::*;

#[test]
fn it_parses_a_struct() {
    let source = include_str!("struct.xs");
    parse_str(source).unwrap();
}
