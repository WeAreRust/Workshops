# Structs - [Chapter 5][rustbook-ch5]

## Exercises

### Velocity

- Create a `Velocity` struct to represent 2D velocity (x and y)
- Implement method for `Velocity` that calculates the magnitude
- Implement a constructor for `Velocity` that takes a direction and magnitude
- Implement the `std::ops::Add` trait for `Velocity` and see what it can do!

### Extension: ECS

- Create a `Movable` struct that owns a `Position`, `Velocity` and a reference to an `Entity` struct (lifetimes)
- Implement `updatePosition` for `Movable` that takes a time
- Create an instance of `Movable` and print its position each second for 10 seconds

