# Default Config

[![Build Status](https://travis-ci.org/kwyse/default-config.svg?branch=master)](https://travis-ci.org/kwyse/default-config)

`default-config` allows you to specify defaults for a `struct` that is intended
to be used as a configuration structure. The defaults you supply can be
overriden by reading in from a YAML file.

## Getting Started

Add this crate to your `Cargo.toml` and ensure you add the `#[macro_use]`
attribute when declaring this crate in `main.rs`/`lib.rs`.

## Usage

Take a look at the tests for the different override combinations that are
supported. The simplest usage would be:

```rust
default_config!(MyConfig, MyConfigDefault, {
    width: u32: 640,
    height: u32: 480
});

let config = MyConfig::merge_with_file("config.yml");
```

If `width` and `height` attributes exist inside the YAML file, the values from
the file will be used. If only `width` exists, the `width` will match the value
in the file, but the `height` will be `480`.

## Limitations and Future Improvements

1. Supplying file overrides for a top-level attribute and a partially-overriden
   nested attribute will cause the whole structure to be defaulted. Ideally the
   macro should be smart enough to still apply the overrides supplied in the
   file.
1. The second argument to the macro is an implementation detail and provides no
   benefit to the user. It should be removed.
1. Support other file types.
