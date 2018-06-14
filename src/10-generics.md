# Generics, Traits, and Lifetimes - [Chapter 10][rustbook-ch10]

[Presentation][presentation]

## Exercises

### Part 1

1. Create a generic function called `largest` which takes a generic `Vec` and returns the item with the largest value.
    - Hint: generic type may have to implement `Ord`.
2. Test this for `i32`, `usize`, and `String` types.

### Part 2

1. Create a generic struct called `Point` which has two attributes of the same type (`x`, and `y`).
2. Create a trait called `MVector` (for matrix-vector), and implement it for `Point`, which has the following method:
    - `dimensions`: returns the number of dimensions.
3. Create a trait called `Magnitude`, and implement it for `Point` which has the following method:
    - `magnitude`: multiplies the square of each attribute.
4. Get the `largest` function to work with `Point`, using `magnitude` to determine the size of the `Point`.

### Part 3

1. Create another generic struct called `Point3` which has three attributes of the same type (`x`, `y`, and `z`).
2. Implement `MVector` for `Point3`.
3. Implement `Magnitude` for `Point3`.

### Extension

1. Write a function that takes a list of items that implement `MVector`, and `Magnitude` and displays dot product as well as the number of dimensions
    - Maybe something like this `[2: 100]`, where `2` is the number of dimensions and `100` is the dot product.
    - Hint: might be easiest to have a `Vec` of `Box` of ...

[rustbook-ch10]: https://doc.rust-lang.org/book/second-edition/ch10-00-generics.html
[presentation]: https://docs.google.com/presentation/d/16dIFfBtF7r-cEi-QYMSziNxixuqKN3t76n6A58ab98Y
