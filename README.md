# utility types

**WIP**

This crate use proc-macro to realize several utility types of TypeScript

## Example

source code:

```rust
#[partial(PartialArticle)]
#[pick(ContentComments, [content, comments], [Debug])]
#[omit(AuthorLikedComments, [content], [])]
pub struct Article<T> {
    author: String,
    content: String,
    liked: usize,
    comments: T,
}
```

generated code:

```rust
pub struct Article<T> {
    author: String,
    content: String,
    liked: usize,
    comments: T,
}
pub struct AuthorLikedComments<T> {
    author: String,
    liked: usize,
    comments: T,
}
#[derive(Debug)]
pub struct ContentComments<T> {
    content: String,
    comments: T,
}
```

## Known Issue

Currently I don't analyze which generic is used in the generated struct or enum. So rustc will complain if the field with generic is not included in the generated struct or enum.
