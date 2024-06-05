# utility types

This crate use proc-macro to realize several utility types of Typescript in Rust.

| macro     | Typescript Utility Type                                                                                                                   |
| --------- | ----------------------------------------------------------------------------------------------------------------------------------------- |
| [Partial] | [Partial\<Type\>](https://www.typescriptlang.org/docs/handbook/utility-types.html#partialtype)                                            |
| [Pick]    | [Pick\<Type, Keys\>](https://www.typescriptlang.org/docs/handbook/utility-types.html#picktype-keys)                                       |
| [Omit]    | [Omit\<Type, Keys\>](https://www.typescriptlang.org/docs/handbook/utility-types.html#omittype-keys)                                       |
| [Extract] | [Extract\<Type, Union\>](https://www.typescriptlang.org/docs/handbook/utility-types.html#extracttype-union)                               |
| [Exclude] | [Exclude\<UnionType, ExcludedMembers\>](https://www.typescriptlang.org/docs/handbook/utility-types.html#excludeuniontype-excludedmembers) |

## Example

Here is an example of how to use this crate.

```rust
use utility_types::{Omit, Partial, Pick, Required};
#[derive(Clone, Partial, Required, Pick, Omit)]
#[partial(ident = PartialFoo, derive(Debug, PartialEq))]
#[required(ident = RequiredFoo, derive(Debug, PartialEq))]
#[pick(arg(ident = PickAB, fields(a, b), derive(Debug, PartialEq)))]
#[omit(arg(ident = OmitCD, fields(c, d), derive(Debug, PartialEq)))]
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

## Known Issue

Currently I don't analyze which generic is used in the generated struct or enum. So rustc will complain if the field with generic is not included in the generated struct or enum.
