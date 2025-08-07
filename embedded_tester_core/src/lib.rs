#![no_std]

#[cfg(test)]
extern crate std;

pub use heapless;

use crate::{
    assertion::{Assertion, AssertionSuccessful},
    error::TestError,
};

pub mod assertion;
pub mod error;
pub mod scheduler;

pub trait TestRunner {
    type Error: core::error::Error;
    type Iterator: core::iter::Iterator<Item = TestResult<'static, Self::Error>>;
    fn execute(self) -> Self::Iterator;
}

pub type TestResult<'a, E> = Result<AssertionSuccessful<'a>, TestError<'a, E>>;

pub struct TestContext<'a, T: Test> {
    assertion: Assertion<'a>,
    test: T,
}
impl<'a, T: Test> TestContext<'a, T> {
    pub fn new(assertion: Assertion<'a>, test: T) -> Self {
        TestContext { assertion, test }
    }
    pub fn run(self) -> TestResult<'a, T::Error> {
        self.test.run(self.assertion)
    }
}

pub trait Test {
    type Error: core::error::Error;
    fn run(self, assertion: Assertion) -> TestResult<'_, Self::Error>;
}
impl<E: core::error::Error> Test for for<'a> fn(Assertion<'a>) -> Result<(), E> {
    type Error = E;
    fn run(self, assertion: Assertion) -> TestResult<'_, Self::Error> {
        let test_name = assertion.test_name();
        if let Err(e) = self(assertion) {
            Err(TestError::new(test_name, e))
        } else {
            Ok(AssertionSuccessful::new(test_name))
        }
    }
}

/*#[derive(Debug)]
pub struct TestResult<'a, const Count: usize> {
    errors: [TestError<'a>; ErrorCount],
}

impl<'a, const ErrorCount: usize> core::fmt::Display for TestResult<'a, ErrorCount> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let mut error_message = String::<ErrorCount>::new();
        for error in self.errors.iter() {
            write!(
                error_message,
                "{}: {}\n",
                error.test_name(),
                error.description()
            );
        }
        let error_count = self.errors.len();
        write!(
            error_message,
            "❌ {}/{} tests succeeded, {error_count} failures!",
            self.total_tests - error_count,
            self.total_tests,
        );
        write!(f, "{}", error_message)
    }
}
impl<'a, const ErrorCount: usize> core::error::Error for TestResult<'a, ErrorCount> {}
*/
