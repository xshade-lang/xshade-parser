extern crate xshade_parser;

use xshade_parser::*;

#[test]
fn it_parses_structs() {
    let source = include_str!("structs.xs");
    parse_str(source).unwrap();
}

#[test]
fn it_parses_functions() {
    let source = include_str!("functions.xs");
    parse_str(source).unwrap();
}
