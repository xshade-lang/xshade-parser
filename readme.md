# xshade-parser

Parser and untyped AST for the xshade language.

## Introduction

`xshade` is a functional shading language targeting SPIR-V. From there SPIR-V tools by khronos can be used to generate glsl or hlsl. This repository contains the parser and resulting abstract syntax tree for xshade.

## Cargo

To use this library, just add it to your Cargo.toml file:

```toml
[dependencies]
xshade_parser = "0.1.0"
```

## Usage

```rust
extern crate xshade_parser;

fn main() {
    let program = "...";
    let result = xshade_parser::parse_str(program);

    match result {
        Ok(ast) => println!("{:#?}", ast),
        Err(error) => println!("{:#?}", error),
    }
}
```

## Getting Help or Contributing

For now, please contact [Marc](https://twitter.com/DottiDeveloper) or [Andy](https://twitter.com/vengarioth) if you need help or want to contribute. We plan to tag some issues with "help wanted" in the future.

## Code of Conduct

We are comitted to provide a friendly, safe and welcoming environment for all, regardless of level of experience, gender identity and expression, sexual orientation, disability, personal appearance, body size, race, ethnicity, age, religion, nationality, or other similar characteristic.

We employ the [Rust Code of Conduct](https://www.rust-lang.org/en-US/conduct.html) where applicable.

## Licence

MIT - If you require this project to be available under a different licence, please open an Issue so we can consider adopting it.
