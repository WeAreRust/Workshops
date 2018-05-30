# Modules - [Chapter 7][rustbook-ch7]

> A module is a namespace that contains definitions of functions or types, and you can choose whether those definitions are visible outside their module (public) or not (private).

## Prelude and Re-exporting

Sometimes we can see

```rust
use std::io::prelude::*;
```

Or something similar.

I was always confused at what `prelude` was and why it was used. But it's actually just a module that re-exports a bunch of modules that are commonly used together.

[`std::io::prelude`][std-io-prelude]

## Super Use

It is possible to `use super::*`. This is particularly useful and common when testing.

```rust
use std::ops::Add;

fn add<T: Add<Output=T>>(a: T, b: T) -> T {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_i32() {
        assert_eq!(add(1i32, 2i32), 3)
    }

    #[test]
    fn add_u32() {
        assert_eq!(add(1u32, 2u32), 3)
    }
}
```

## Nested Use

We already know that you can do this:

```rust
use std::cmp::{PartialEq, Eq};
use std::os::linux::fs;
use std::os::windows::fs;
```

But (thanks to [RFC2128][rfc2128]) we can also do this:

```rust
use std::{
    cmp::{Eq, PartialEq}, os::{linux::fs, windows::fs},
};
```

[rustbook-ch7]: https://doc.rust-lang.org/book/second-edition/ch07-00-modules.html
[rfc2128]: https://github.com/rust-lang/rfcs/blob/master/text/2128-use-nested-groups.md
[std-io-prelude]: https://doc.rust-lang.org/std/io/prelude/index.html
