# Clutch

A modern & minimal programming language.

```txt
let main = () => {
    print('Hello World!')
}
```

## Requirements

- [Rust](https://www.rust-lang.org/), see [`Cargo.toml`](Cargo.toml) for the
  minimum version.

## Usage

This is a very experimental project, so abandon all hope ye who enter here.

All this project does right now is start a REPL that lexes the input and prints
the tokens:

```sh
$ cargo run

Welcome to the Clutch REPL!
TIP: Press Ctrl-C to exit.

> let main = () => print('Hello World')
  Identifier("let")
  Identifier("main")
  Equals
  OpenParenthesis
  CloseParenthesis
  Equals
  Identifier("print")
  Literal(String("Hello World"))
  CloseParenthesis
```
