# matrix-rust

A general-use linear algebra library in Rust. Done to complete the 42 `matrix` project.

## Main features

- Dynamically-sized matrix and vectors, with useful error types.
- Static-sized matrix and vectors, with compile-time check to prevent any misuse of the functions, thus removing all need of error types.
- Complex numbers

## Use cases

In almost all situations, static-sized matrix and vector should be prefered over their dynamic counterpart. The only reason to use dynamic-sized matrices and vectors should be to treat user input. In all other contexts, the static-sized version offers far more convenience.

## About the use of unsafe

In a few files, you'll see the use of unsafe functions.
They're always carefully checked, and explained clearly.
