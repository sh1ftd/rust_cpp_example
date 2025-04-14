# Rust-CPP example

A simple demonstration of calling C++ code from Rust using FFI (Foreign Function Interface). This is a demo project which counts prime numbers up to a given number using the Sieve of Eratosthenes algorithm implemented in C++.

## Requirements

- Rust (2024 edition)
- C++17 compatible compiler
- Windows: Visual Studio Build Tools with C++ support

## Building

```bash
cargo build --release
```

## Credits

- Uses [cc-rs](https://github.com/rust-lang/cc-rs) for C++ compilation and linking

## License

MIT
