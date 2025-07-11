# utility types

[![crates.io](https://img.shields.io/crates/v/utility-types.svg)](https://crates.io/crates/utility-types)
[![docs.rs](https://docs.rs/utility-types/badge.svg)](https://docs.rs/utility-types)

This crate use proc-macro to realize several utility types of Typescript in Rust.

| macro      | Typescript Utility Type                                                                                                                   |
| ---------- | ----------------------------------------------------------------------------------------------------------------------------------------- |
| [Partial]  | [Partial\<Type\>](https://www.typescriptlang.org/docs/handbook/utility-types.html#partialtype)                                            |
| [Required] | [Required\<Type\>](https://www.typescriptlang.org/docs/handbook/utility-types.html#requiredtype)                                          |
| [Pick]     | [Pick\<Type, Keys\>](https://www.typescriptlang.org/docs/handbook/utility-types.html#picktype-keys)                                       |
| [Omit]     | [Omit\<Type, Keys\>](https://www.typescriptlang.org/docs/handbook/utility-types.html#omittype-keys)                                       |
| [Extract]  | [Extract\<Type, Union\>](https://www.typescriptlang.org/docs/handbook/utility-types.html#extracttype-union)                               |
| [Exclude]  | [Exclude\<UnionType, ExcludedMembers\>](https://www.typescriptlang.org/docs/handbook/utility-types.html#excludeuniontype-excludedmembers) |

## Example

Here is an example of how to use this crate.

```rust
use utility_types::{Omit, Partial, Pick, Required};
#[derive(Clone, Partial, Required, Pick, Omit)]
#[partial(ident = PartialFoo, derive(Debug, PartialEq), forward_attrs())]
#[required(ident = RequiredFoo, derive(Debug, PartialEq), forward_attrs())]
#[pick(arg(ident = PickAB, fields(a, b), derive(Debug, PartialEq)), forward_attrs())]
#[omit(arg(ident = OmitCD, fields(c, d), derive(Debug, PartialEq)), forward_attrs())]
pub struct Foo {
    a: u8,
    b: Option<u8>,
    c: Option<Vec<u8>>,
}
```

The above code will generate the following code.

```rust
#[derive(Debug, PartialEq)]
pub struct PartialFoo {
    a: Option<u8>,
    b: Option<Option<u8>>,
    c: Option<Option<Vec<u8>>>,
}
#[derive(Debug, PartialEq)]
pub struct RequiredFoo {
    a: u8,
    b: u8,
    c: Vec<u8>,
}
#[derive(Debug, PartialEq)]
pub struct PickAB {
    a: u8,
    b: Option<u8>,
}
#[derive(Debug, PartialEq)]
pub struct OmitCD {
    a: u8,
    b: Option<u8>,
}
```

Some useful traits are also generated:

- `From<Foo>` for `PartialFoo`, `PickAB`, `OmitCD`
- `From<PartialFoo>` for `Foo`

### Forwarding Attributes

To use this crate with other crates that need attributes, you can use the `forward_attrs` attribute to control which attributes are forwarded to the generated struct or enum.

```rust
use serde::{Deserialize, Serialize};
use utility_types::Omit;

#[derive(Debug, PartialEq, Serialize, Deserialize, Omit)]
#[omit(
    arg(
        ident = OmitCD,
        fields(c, d),
        derive(Debug, PartialEq, Serialize, Deserialize),
        forward_attrs(serde)
    )
)]
#[serde(rename_all = "UPPERCASE")]
pub struct Foo {
    a: u8,
    b: Option<u8>,
    c: Option<Vec<u8>>,
}

let omit_cd: OmitCD = serde_json::from_str(r#"{"A": 1, "B": 2}"#).unwrap();
assert_eq!(omit_cd, OmitCD { a: 1, b: Some(2) });
```

The behavior of the `forward_attrs` attribute is as follows:

- If **not provided**, all attributes are forwarded by default.
- If provided with a list of attributes, only the specified attributes are forwarded.
  - For example, `forward_attrs(doc, serde)` will forward only `doc` and `serde`.
  - If provided with **only `*`** (`forward_attrs(*)`), all attributes are forwarded.
  - If provided with **an empty list** (`forward_attrs()`), no attributes are forwarded.
- If provided with a list inside `not()`, all attributes except the specified attributes are forwarded.
  - For example, `forward_attrs(not(serde))` will forward all attributes except `serde`.

## Known Issue

Currently I don't analyze which generic is used in the generated struct or enum. So rustc will complain if the field with generic is not included in the generated struct or enum.
