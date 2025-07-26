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

pub trait TestRunner<const ErrorDescriptionLength: usize> {
    fn execute<'a>(self) -> impl Iterator<Item = TestResult<'a, ErrorDescriptionLength>> + 'a;
}

pub type TestResult<'a, const ErrorDescriptionLength: usize> =
    Result<AssertionSuccessful<'a, ErrorDescriptionLength>, TestError<'a, ErrorDescriptionLength>>;

pub struct TestContext<'a, const ErrorDescriptionLength: usize, T: Test<ErrorDescriptionLength>> {
    assertion: Assertion<'a, ErrorDescriptionLength>,
    test: T,
}
impl<'a, const ErrorDescriptionLength: usize, T: Test<ErrorDescriptionLength>>
    TestContext<'a, ErrorDescriptionLength, T>
{
    pub fn new(assertion: Assertion<'a, ErrorDescriptionLength>, test: T) -> Self {
        TestContext { assertion, test }
    }
    pub fn run(self) -> TestResult<'a, ErrorDescriptionLength> {
        self.test.run(self.assertion)
    }
}

pub trait Test<const ErrorDescriptionLength: usize> {
    fn run(
        self,
        assertion: Assertion<ErrorDescriptionLength>,
    ) -> TestResult<'_, ErrorDescriptionLength>;
}
impl<const ErrorDescriptionLength: usize> Test<ErrorDescriptionLength>
    for for<'a> fn(Assertion<'a, ErrorDescriptionLength>) -> TestResult<'a, ErrorDescriptionLength>
{
    fn run(
        self,
        assertion: Assertion<ErrorDescriptionLength>,
    ) -> TestResult<'_, ErrorDescriptionLength> {
        self(assertion)
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
