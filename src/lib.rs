#[macro_use] extern crate serde_derive;
extern crate serde;
extern crate serde_yaml;

use std::io::Read;
use std::fs::File;
use std::path::Path;

macro_rules! default_config {
    ( 
        $name:ident, $default:ident, {
            $( $id:ident: $type:ty: $value:expr ),*
        }
    ) => {
        #[derive(Serialize, Deserialize)]
        pub struct $name {
            $(
                $id: $type,
            )*
        }

        #[derive(Serialize, Deserialize)]
        struct $default {
            $(
                $id: Option<$type>,
            )*
        }

        impl $name {
            pub fn from_file<P: AsRef<Path>>(path: P) -> Self {
                match File::open(path) {
                    Ok(file) => Self::from_reader(file),
                    Err(_) => Self::default()
                }
            }

            fn from_reader<R: Read>(reader: R) -> Self {
                match serde_yaml::from_reader::<R, $default>(reader) {
                    Ok(overrides) => {
                        Self {
                            $(
                                $id: overrides.$id.unwrap_or($name::default().$id),
                            )*
                        }
                    },
                    Err(_) => Self::default()
                }
            }
        }

        impl Default for $name {
            fn default() -> Self {
                Self {
                    $(
                        $id: $value,
                    )*
                }
            }
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn no_defaults_are_overriden() {
        default_config!(Test, TestDefault, {
            foo: i64: 42,
            bar: u16: 451
        });

        let buffer = Cursor::new("");
        let config = Test::from_reader(buffer);
        assert_eq!(config.foo, 42_i64);
        assert_eq!(config.bar, 451_u16);
    }

    #[test]
    fn all_defaults_are_overriden() {
        default_config!(TestInner, TestInnerDefault, {
            foo: i64: 42,
            bar: u16: 451
        });

        default_config!(Test, TestDefault, {
            baz: String: String::from("baz"),
            quux: TestInner: TestInner::default()
        });

        let buffer = Cursor::new("baz: real baz\nquux:\n  foo: 10\n  bar: 20");
        let config = Test::from_reader(buffer);
        assert_eq!(config.baz, String::from("real baz"));
        assert_eq!(config.quux.foo, 10);
        assert_eq!(config.quux.bar, 20);
    }

    #[test]
    fn top_level_defaults_are_overriden() {
        default_config!(TestInner, TestInnerDefault, {
            foo: i64: 42,
            bar: u16: 451
        });

        default_config!(Test, TestDefault, {
            baz: String: String::from("baz"),
            quux: TestInner: TestInner::default()
        });

        let buffer = Cursor::new("baz: real baz");
        let config = Test::from_reader(buffer);
        assert_eq!(config.baz, String::from("real baz"));
        assert_eq!(config.quux.foo, 42);
        assert_eq!(config.quux.bar, 451);
    }

    #[test]
    fn nested_defaults_are_overriden() {
        default_config!(TestInner, TestInnerDefault, {
            foo: i64: 42,
            bar: u16: 451
        });

        default_config!(Test, TestDefault, {
            baz: String: String::from("baz"),
            quux: TestInner: TestInner::default()
        });

        let buffer = Cursor::new("quux:\n  foo: 10\n  bar: 20");
        let config = Test::from_reader(buffer);
        assert_eq!(config.baz, String::from("baz"));
        assert_eq!(config.quux.foo, 10);
        assert_eq!(config.quux.bar, 20);
    }
}
