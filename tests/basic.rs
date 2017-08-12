#[macro_use]
extern crate specified_default_derive;

#[test]
fn one_field_without_default() {
    #[derive(SpecifiedDefault)]
    struct Foo {
        bar: u32,
    }

    assert_eq!(Foo::default().bar, 0);
}

#[test]
fn one_field_with_default() {
    #[derive(SpecifiedDefault)]
    struct Foo {
        #[default = "42"]
        bar: u32,
    }

    assert_eq!(Foo::default().bar, 42);
}

#[test]
fn fields_with_and_without_defaults() {
    #[derive(SpecifiedDefault)]
    struct Foo {
        bar: u32,

        #[default = "42"]
        baz: u32
    }

    assert_eq!(Foo::default().bar, 0);
    assert_eq!(Foo::default().baz, 42);
}

#[test]
fn numeric_defaults() {
    #[derive(SpecifiedDefault)]
    struct Foo {
        #[default = "1"]
        int8: i8,
        #[default = "2"]
        int16: i16,
        #[default = "3"]
        int32: i32,
        #[default = "4"]
        int64: i64,
        #[default = "5"]
        uint8: u8,
        #[default = "6"]
        uint16: u16,
        #[default = "7"]
        uint32: u32,
        #[default = "8"]
        uint64: u64,
        #[default = "9.5"]
        float32: f32,
        #[default = "10.451"]
        float64: f64,
    }

    let result = Foo::default();
    assert_eq!(result.int8, 1);
    assert_eq!(result.int16, 2);
    assert_eq!(result.int32, 3);
    assert_eq!(result.int64, 4);
    assert_eq!(result.uint8, 5);
    assert_eq!(result.uint16, 6);
    assert_eq!(result.uint32, 7);
    assert_eq!(result.uint64, 8);
    assert_eq!(result.float32, 9.5);
    assert_eq!(result.float64, 10.451);
}

#[test]
fn string_defaults() {
    #[derive(SpecifiedDefault)]
    struct Foo {
        #[default = "the bar field"]
        bar: String,
    }

    assert_eq!(Foo::default().bar, "the bar field".to_string());
}

#[test]
fn struct_defaults() {
    #[derive(SpecifiedDefault)]
    struct Foo {
        #[default = "42"]
        bar: u32,

        baz: u32,
    }

    #[derive(SpecifiedDefault)]
    struct Quux {
        foo: Foo,

        #[default = "451"]
        bar: u32,
    }

    let result = Quux::default();
    assert_eq!(result.foo.bar, 42);
    assert_eq!(result.foo.baz, 0);
    assert_eq!(result.bar, 451);
}

#[test]
#[should_panic(expected = "Failed to parse")]
fn fail_parsing() {
    #[derive(SpecifiedDefault)]
    struct Foo {
        #[default = "42s"]
        bar: u32,
    }

    Foo::default().bar;
}
