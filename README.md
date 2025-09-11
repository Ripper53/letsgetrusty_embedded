# embedded_tester

This follows [Let's Get Rusty's embedded test runner project](https://github.com/letsgetrusty/bootcamp/tree/master/4.%20Projects/4.Embedded/Problem).
This project was created with the intent to learn about Rust's `no_std` and stack-based programming.

## About
Stack-based `no_std` test runner. Does not allocate memory on the heap.

## Example Project
`embedded_runner` is an example project.
Read its `README.md` [here](embedded_runner/README.md).

## How to Use
A `TestRunner` holds function pointers of signature `fn(Assertion) -> Result<(), Error>` where `Error` is a type that implements `core::error::Error`.
The main way to use this is with the `TestRunner` and `TestScheduler` derive macros.
`TestRunner` is your main derive macro, which holds function pointers that act as tests.
To create suites of tests, use `TestScheduler` which can hold other `TestScheduler`s and `TestRunner`s.

Example code:
```rust
// The max length of the error message
// used by `AssertionFailure`.
// This max length is required because
// it will be allocated on the stack.
// Strings longer than this will be truncated.
const ERROR_SIZE: usize = 255;
#[derive(TestRunner)]
struct Tests {
    // Explicit lifetime, and can only catch assertion failures.
    test_a: for<'a> fn(Assertion<'a>) -> Result<(), AssertionFailure<ERROR_SIZE>>,
    // Custom error type,
    // if you wish to allow this test to also catch an `AssertionFailure`,
    // make a new error that can be converted with the `From` trait
    // to this error and `AssertionFailure`.
    test_b: fn(Assertion) -> Result<(), CustomError>,
}

fn test_a(assertion: Assertion<'_>) -> Result<(), AssertionFailure<ERROR_SIZE>> {
    assertion.assert_eq(1, 1)?;
    Ok(())
}

fn test_b(_assertion: Assertion) -> Result<(), CustomError> {
    Err(CustomError::ErrorA)
}

#[derive(Debug)]
enum CustomError {
    // VARIANTS
}
impl Error for CustomError {}
impl Display for CustomError {
    // IMPL
}
```

This will implement the `TestRunner::execute` method that can be called on `Tests` like so:
```rust
for test_result in Tests::default().execute() {
    // IMPL
}
```
`TestScheduler` derive macro implements `TestScheduler::execute_suites` method and `TestRunner::execute`, and can only hold `TestRunner`s and other `TestScheduler`s.

## Test Code
We use Docker to build the `embedded_runner` project with different QEMU simulated devices.
Run `docker-compose up` in the root of this repository. It will:
1. Compile the project multiple times in release mode with different amounts of memory
2. Run the tests found in the project with the specifications and once with CPU throttle
