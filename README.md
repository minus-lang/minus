# The Minus Programming Language
Minus is a minimalistic programming language that emphasizes simplicity, readability, and conciseness. It is designed for developers who want to write code with less syntax noise and more readability.

## Features
- Statically typed
- Immutable variables by default
- Indentation-based code blocks
- Simple and consistent syntax

## Getting Started
To get started with Minus, you need to have Rust installed on your machine.

## Hello World
Here's a simple "Hello, World!" program in Minus:

```
fn -
    > 'Hello, World!'
```

## Examples
Take a look at the examples directory for more sample programs.

## Documentation
The documentation for Minus can be found in the wiki.

## Contributing
We welcome contributions from the community. If you are interested in contributing to Minus, please take a look at the CONTRIBUTING.md file for more information.

## License
Minus is licensed under the terms of both the MIT license and the Apache License (Version 2.0).

See [LICENSE-APACHE](LICENSE-APACHE) and [LICENSE-MIT](LICENSE-MIT) for details.

## TODO
- [ ] Define the syntax and grammar of the language: This includes creating a language specification document that outlines the rules of the language, including keywords, operators, data types, and so on.
- [ ] Develop a lexer and parser: The lexer is responsible for breaking the code down into tokens, and the parser is responsible for interpreting these tokens and creating an abstract syntax tree (AST) that represents the code.
- [ ] Implement a code generator: This is the component that converts the AST into the target language (in this case Rust).
- [ ] Develop a standard library: Implement commonly used functionality such as input/output, data structures, and algorithms.
- [ ] Create a development environment: Develop a set of tools, such as a text editor or integrated development environment (IDE), to make it easier for developers to write and test code.
- [ ] Document and promote the language: Create documentation and tutorials to help developers learn and use the language.

## check these crates to use
- The LALRPOP library is a popular choice for implementing a parser in Rust. It uses LALR(1) parsing and allows for the definition of the grammar in Rust code.
- The nom library provides a set of macros to help in the implementation of parsers and lexers.
- The rust-peg library provides a PEG (Parsing Expression Grammar) implementation for Rust.
- The combine library provides a general parser combinator framework for Rust
