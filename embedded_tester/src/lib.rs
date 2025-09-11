//! `TestRunner` derive macro requires each field to be a function pointer
//! with the function signature `fn(Assertion) -> Result<(), Error>`
//! where `Error` is a type that implements `core::error::Error`.
//!
//! Example:
//! ```
//! use embedded_tester::{
//!     TestScheduler, TestRunner, TestResult, TestLogger,
//!     assertion::{Assertion, AssertionFailure},
//! };
//!
//! fn main() {
//!     // Indiviually run test runner
//!     for test_result in TestRunnerA::default().execute() {
//!         assert!(test_result.is_ok());
//!     }
//!     
//!     // Run test scheduler
//!     for test_result in TestSchedulerA::default().execute() {
//!         // Code
//!     }
//!     
//!     // Logger for logging test suites
//!     struct Logger;
//!     impl TestLogger for Logger {
//!         fn log_test_suite_name(&self, name: &str) {
//!             println!("{name}");
//!         }
//!         fn log_test_result<E: core::error::Error>(&self, test_result: TestResult<'_, E>) {
//!             match test_result {
//!                 Ok(success) => println!("passed {}", success.test_name()),
//!                 Err(e) => println!("failed {e}"),
//!             }
//!         }
//!     }
//!     TestSchedulerA::default().execute_suites(Logger);
//! }
//!
//! // Test scheduler requires `scheduler` feature
//! // which is enabled by default
//! #[derive(TestScheduler)]
//! struct TestSchedulerA {
//!     test_runner_a: TestRunnerA,
//! }
//!
//! #[derive(TestScheduler)]
//! struct TestSchedulerB {
//!     // TestScheduler implements TestRunner,
//!     // so they can be nested.
//!     test_scheduler_a: TestSchedulerA,
//!     test_runner_b: TestRunnerB,
//! }
//!
//! const ERROR_SIZE: usize = 255;
//! #[derive(TestRunner)]
//! struct TestRunnerA {
//!     // Test fails if an assertion failure is returned
//!     test_a: for<'a> fn(Assertion<'a>) -> Result<(), AssertionFailure<ERROR_SIZE>>,
//! }
//!
//! impl Default for TestRunnerA {
//!     fn default() -> Self {
//!         TestRunnerA {
//!             test_a,
//!         }
//!     }
//! }
//!
//! #[derive(TestRunner)]
//! struct TestRunnerB {
//!     // Custom error which can be a cause of failure for the test
//!     test_b: fn(Assertion<'_>) -> Result<(), CustomError>,
//! }
//!
//! impl Default for TestRunnerB {
//!     fn default() -> Self {
//!         TestRunnerB {
//!             test_b,
//!         }
//!     }
//! }
//!
//! fn test_a(assertion: Assertion<'_>) -> Result<(), AssertionFailure<ERROR_SIZE>> {
//!     assertion.assert_eq(1, 1)?;
//!     Ok(())
//! }
//!
//! fn test_b(assertion: Assertion<'_>) -> Result<(), CustomError> {
//!     Err(CustomError::VariantA)
//! }
//!
//! #[derive(Debug)]
//! enum CustomError {
//!     VariantA,
//!     VariantB,
//! }
//! impl core::fmt::Display for CustomError {
//!     fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
//!         match self {
//!             CustomError::VariantA => write!(f, "FIRST_ERROR"),
//!             CustomError::VariantB => write!(f, "SECOND_ERROR"),
//!         }
//!     }
//! }
//! impl core::error::Error for CustomError {}
//! ```

#![no_std]
pub use embedded_tester_core::*;
pub use embedded_tester_derive::*;
