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

pub trait TestRunner {
    fn execute(self);
}
/*pub struct TestRunner<'a, const COUNT: usize> {
    tests: [TestContext<'a>; COUNT],
}*/
#[macro_export]
macro_rules! new_test_runner {
    ($test_runner_name: ident, $(($test_name: literal, $test_fn: expr)),+) => {
        pub struct $test_runner_name<'a> {
            $($test_name: $test_fn)*
        }
        impl<'a> $test_runner_name<'a> {
            pub fn add_test(&mut self, name: &'a str, test: impl Into<Test>) {
                self.tests.push(TestContext {
                    name,
                    test: test.into(),
                });
            }
            pub fn execute(self) {
                if let Err(errors) = self.run() {
                    panic!("{errors}");
                } else {
                }
            }
            fn run(self) -> Result<(), TestErrors<'a, COUNT>> {
                let total_tests = self.tests.len();
                let mut errors: Vec<_, COUNT> = self
                    .tests
                    .into_iter()
                    .filter_map(|test| {
                        if let Err(e) = test.run() {
                            Some(e)
                        } else {
                            None
                        }
                    })
                    .collect();
                if errors.is_empty() {
                    Ok(())
                } else {
                    Err(TestErrors {
                        total_tests,
                        errors,
                    })
                }
            }
        }
    };
}
pub struct TestContext<'a, const ErrorDescriptionLength: usize> {
    name: &'a str,
    test: Test<ErrorDescriptionLength>,
}
impl<'a, const ErrorDescriptionLength: usize> TestContext<'a, ErrorDescriptionLength> {
    pub fn new(name: &'a str, test: Test<ErrorDescriptionLength>) -> Self {
        TestContext { name, test }
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn run(
        self,
        test_error: TestError<'a, ErrorDescriptionLength>,
    ) -> Result<(), TestError<'a, ErrorDescriptionLength>> {
        self.test.run(Assertion::new(self.name, test_error))
    }
}
pub type TestResult<'a, const ErrorDescriptionLength: usize> =
    Result<(), TestError<'a, ErrorDescriptionLength>>;
pub struct Test<const ErrorDescriptionLength: usize>(
    for<'a> fn(Assertion<'a, ErrorDescriptionLength>) -> TestResult<'a, ErrorDescriptionLength>,
);
impl<const ErrorDescriptionLength: usize> Test<ErrorDescriptionLength> {
    pub fn run<'a>(
        self,
        assertion: Assertion<'a, ErrorDescriptionLength>,
    ) -> Result<(), TestError<'a, ErrorDescriptionLength>> {
        (self.0)(assertion)
    }
}
impl<const ErrorDescriptionLength: usize>
    From<
        for<'a> fn(Assertion<'a, ErrorDescriptionLength>) -> TestResult<'a, ErrorDescriptionLength>,
    > for Test<ErrorDescriptionLength>
{
    fn from(
        value: for<'a> fn(
            Assertion<'a, ErrorDescriptionLength>,
        ) -> TestResult<'a, ErrorDescriptionLength>,
    ) -> Self {
        Test(value)
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
