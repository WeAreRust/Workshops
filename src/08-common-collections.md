# Common Collections - [Chapter 8][rustbook-ch8]

## Exercises

### Statistics

Create the following `Vec`:

```rust
let v = vec![1, 5, 4, 2, 7, 2]
```

Now find and display:

- The average
- The median (middle value when sorted)
- The mode (value that occurs most often)

### Pig Latin

Convert string to Pig Latin.

This works by taking the first consonant of each work and moving it to the end of the word, adding the suffix "ay". If the word starts with a vowel then leave it unchanged and add the prefix "hay". For example:

- `first -> irst-fay`
- `apple -> apple-hay`

### Extension: HashSet

Implement `HashSet` as a wrapper around `HashMap`, for `Strings`.

Implement the following methods:

- `fn new() -> HashSet`
- `fn add(&self, item: String)`
- `fn remove(&self, item: String)`
- `fn contains(&self, item: &str) -> bool`

#### Extension Extension

Also implement these methods on your `HashSet`.

- `fn union(&self, other: &HashSet) -> HashSet`
- `fn intersection(&self, other: &HashSet) -> HashSet`

[rustbook-ch8]: https://doc.rust-lang.org/book/second-edition/ch08-00-common-collections.html

