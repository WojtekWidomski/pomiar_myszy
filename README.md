# Mouse pointing time measuring program

Simple program to measure user pointing time written in Rust.

It shows black squares at random positions on screen and prints initial distance
from cursor and time to stdout in csv format. It starts measuring time after
user starts moving mouse (not immediately after displaying target), so it really
measures pointing time and not reaction time. It can ignore first few clicks.

## Usage

```
cargo run --relase
```

If you want to save output to file:

```
cargo run --release > output.csv
```

Use constants at the beginning of `main.rs` to change options of this program.

## Used technologies

Please see `Cargo.toml` for list of dependencies.

- Rust programming language
- [Piston graphics library](https://github.com/PistonDevelopers/graphics)
- glfw
