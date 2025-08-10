# letsgetrusty_embedded

This follows [Let's Get Rusty's embedded test runner project](https://github.com/letsgetrusty/bootcamp/tree/master/4.%20Projects/4.Embedded/Problem).
This project was created with the intent to learn about Rust's `no_std` and stack-based programming.

## About
Stack-based `no_std` test runner.

## Examples

A `TestRunner` holds function pointers of signature `fn(Assertion) -> Result<(), impl Error>,
```rust
// The max length of the error message
// relative to `AssertionFailure`
const ERROR_SIZE: usize = 255;
#[derive(TestRunner)]
struct Tests {
    // Explicit lifetime
    test_a: for<'a> fn(Assertion<'a>) -> Result<(), AssertionFailure<ERROR_SIZE>>,
    // Custom error type
    test_b: fn(Assertion) -> Result<(), CustomError>,
}

fn test_a(assertion: Assertion<'_>) -> Result<(), AssertionFailure<ERROR_SIZE>> {
    assertion.assert_eq(1, 1)?;
    Ok(())
}

fn test_b(assertion: Assertion) -> Result<(), CustomError> {
    Err(CustomError::ErrorA)
}

#[derive(Debug)]
enum CustomError {
    // VARIANTS
}

impl Error for CustomError {
    // IMPL
}
```
## Example Project
`embedded_runner` is an example project.
Read its `README.md` [here](embedded_runner/README.md).
