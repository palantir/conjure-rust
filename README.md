# Conjure-Rust [![CircleCI](https://circleci.com/gh/palantir/conjure-rust.svg?style=shield)](https://circleci.com/gh/palantir/conjure-rust)

_[Conjure](https://github.com/palantir/conjure) support in Rust._

## Usage

`conjure-rust` is an [RFC 002](
https://github.com/palantir/conjure/blob/develop/rfc/002-contract-for-conjure-generators.md)-compliant CLI, which can
be used via a build tool like [gradle-conjure](https://github.com/palantir/gradle-conjure), or manually:

```
USAGE:
    conjure-rust generate [OPTIONS] <input-json> <output-directory>

OPTIONS:
        --exhaustive                     Generate exhaustively matchable enums and unions
        --conjure-path <conjure_path>    The module path to the conjure crate root
    -h, --help                           Prints help information
    -V, --version                        Prints version information

ARGS:
    <input-json>          Path to a JSON-formatted Conjure IR file
    <output-directory>    Directory to place generated code
```

Alternatively, the `conjure-codegen` library can be used to programmatically generate code in a build script:

```rust
use std::env;
use std::path::PathBuf;

fn main() {
    let input = concat!(env!("CARGO_MANIFEST_DIR"), "/service-api.conjure.json");
    println!("cargo:rerun-if-changed={}", input);
    conjure_codegen::Config::new()
        .generate_files(
            input,
            PathBuf::from(env::var_os("OUT_DIR").unwrap()).join("conjure"),
        )
        .unwrap();
}
```

### Options

The code generation can be controlled via a small number of options:

* `exhaustive` - Normally, generated enums and unions are be able to deserialize and pass along unknown variants, but
    this means that you cannot exhaustively match on them. If you instead want deserialization to return an error when
    encountering an unknown variant, enable this option.

## Example generated objects

- **Conjure object: [ManyFieldExample](https://docs.rs/conjure/*/conjure/example_types/struct.ManyFieldExample.html)**

    Objects are represented as Rust structs. They are constructed using the builder pattern, and provide accessors for
    their fields.

    ```rust
    let object = ManyFieldExample::builder()
        .string("foo")
        .integer(123)
        .optional_item("bar".to_string())
        .items(iterator_of_items)
        .build();

    assert_eq!(object.string(), "foo");
    assert_eq!(object.optional_item(), Some("bar"));
    ```

- **Conjure union: [UnionTypeExample](https://docs.rs/conjure/*/conjure/example_types/enum.UnionTypeExample.html)**

    Unions are represented as Rust enums. In addition to the specified variants, the enum by default contains an extra
    `Unknown` variant used to pass-through variants which are not in the union definition.

    ```rust
    match union_value {
        UnionTypeExample::StringExample(string) => {
            // ...
        }
        UnionTypeExample::Set(set) => {
            // ...
        }
        // ...
        UnionTypeExample::Unknown(unknown) => {
            println!("encountered unknown variant: {}", unknown.type_());
        }
    }
    ```

- **Conjure enum: [EnumExample](https://docs.rs/conjure/*/conjure/example_types/struct.EnumExample.html)**

    Enums are *not*, as you might expect, represented as Rust enums. Instead, they are structs with associated constants
    representing the variants. This allows them to be used like an enum would, but also be forwards compatible by being
    able to deserialize and pass-through unknown variants.

    ```rust
    match enum_value {
        EnumExample::ONE => println!("found one"),
        EnumExample::TWO => println!("found two"),
        other => println!("found unknown variant: {}", other),
    }
    ```

- **Conjure alias: [StringAliasExample](https://docs.rs/conjure/*/conjure/example_types/struct.StringAliasExample.html)**

    Aliases are represented as Rust newtype structs. Aliases serialize and deserialize like their inner types do, so
    they are useful for adding extra type-safety to APIs.

    ```diff
    -do_something(product_id: String, user_id: String, email_address: String);
    +do_something(product_id: ProductId, user_id: UserId, email_address: EmailAddress);
    ```
