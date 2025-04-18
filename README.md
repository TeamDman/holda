# Holda

Holda is a Rust library that provides convenience macros for creating domain objects, especially those that wrap existing types, with built-in support for `serde` serialization and deserialization. It simplifies the process of creating value types with common trait implementations, reducing boilerplate and improving code readability.

## Features

*   **Automatic Trait Implementations:**  The `StringHolda` and `Holda` derive macros automatically implement common traits like `From`, `AsRef`, `Deref`, `Display`, `Debug`, `PartialEq`, `Eq`, `PartialOrd`, `Ord`, `Hash`, and `Clone`.
*   **`serde` Support:**  Easily enable `serde` serialization and deserialization for your wrapper types with the `serde` feature.
*   **Customizable:**  Skip specific trait implementations using the `#[holda(...)]` attribute.
*   **String-Specific Macro:** The `StringHolda` macro is optimized for creating wrappers around `String` types, providing `FromStr` implementations.

## Usage

Add `holda` to your `Cargo.toml`:

```toml
[dependencies]
holda = "0.1.0"
```

### `StringHolda` Macro

The `StringHolda` macro is designed for structs that hold a `String`.  It automatically implements traits and provides `FromStr` implementation.

```rust
use holda::StringHolda;

#[derive(StringHolda)]
struct UserName {
    inner: String,
}

fn main() {
    let name = "bruh";
    let user_name: UserName = name.parse().unwrap();
    assert_eq!(*user_name, name);
    let user_name = UserName::from(name);
    assert_eq!(*user_name, name);
}
```

With the `serde` feature enabled, you can serialize and deserialize the `UserName` struct:

```rust
use holda::StringHolda;
use serde::Deserialize;
use serde::Serialize;

#[derive(StringHolda, Serialize, Deserialize, Debug, PartialEq)]
struct UserName {
    inner: String,
}

fn main() {
    let data = r#"
    {
        "inner": "Bruh"
    }
    "#;
    let user_name: UserName = serde_json::from_str(data).unwrap();
    println!("{:?}", user_name);
    assert_eq!(*user_name, "Bruh");
}
```

### `Holda` Macro

The `Holda` macro is more generic and can be used with any type.  It requires specifying the field name as `inner`.

```rust
use holda::Holda;
use uuid::Uuid;

#[derive(Holda)]
struct MyUuidWrapper {
    inner: Uuid,
}

fn main() {
    let uuid = Uuid::new_v4();
    let wrapper = MyUuidWrapper::new(uuid);

    assert_eq!(*wrapper, uuid);
}
```

You can skip trait implementations using the `#[holda(...)]` attribute:

```rust
use holda::Holda;

#[derive(Holda)]
#[holda(NoDisplay, NoEq, NoOrd, NoHash)]
struct MyUnitWrapper {
    inner: (),
}

fn main() {
    let wrapper = MyUnitWrapper::new(());
    assert_eq!(*wrapper, ());
}
```

### `serde` Feature

To enable `serde` support, add the `serde` feature to your `Cargo.toml`:

```toml
[dependencies]
holda = { version = "0.1.0", features = ["serde"] }
serde = { version = "1.0", features = ["derive"] } # required
serde_json = "1.0" # required for testing
```

Now, the generated code will include `serde::Serialize` and `serde::Deserialize` implementations for your wrapper types.

## Skipping Trait Implementations

You can use the `#[holda(...)]` attribute to skip specific trait implementations.  The following options are available:

*   `NoDisplay`: Skips implementing `Display`.
*   `NoEq`: Skips implementing `PartialEq` and `Eq`.
*   `NoOrd`: Skips implementing `PartialOrd` and `Ord`.
*   `NoHash`: Skips implementing `Hash`.
*   `NoClone`: Skips implementing `Clone`.
    *   `NoSerde`: Skips implementing `Serialize` and `Deserialize`.

## License

This project is licensed under the MPL-2.0 License. See the [LICENSE](LICENSE) file for details.