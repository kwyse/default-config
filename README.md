# Specified Default

[![Build Status](https://travis-ci.org/kwyse/specified-default-derive.svg?branch=master)](https://travis-ci.org/kwyse/specified-default-derive)

`specified_default_derive` enables you to choose the defaults implemented by the
`Default` trait. After it is derived, the object will behave exactly as it would
had the standard `Default` trait been derived. Nested objects must implement
either `SpecifiedDefault` or `Default`.

If you don't provide an override, the existing default value for that type will
be used.

## Getting Started

Add this crate to your `Cargo.toml` and ensure you add the `#[macro_use]`
attribute when declaring this crate in `main.rs`/`lib.rs`.

## Usage

Take a look at the tests for the different override combinations that are
supported. The simplest usage would be:

```rust
#[derive(SpecifiedDefault)]
struct MyStruct {
    #[default = "640"]
    width: u32,
    #[default = "480"]
    height: u32,

    scenes: u32,
}

let result = MyStruct::default();

assert_eq!(result.width, 640);
assert_eq!(result.height, 480);
assert_eq!(result.scenes, 0);
```
