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
#[cfg(feature = "scheduler")]
mod scheduler;
#[cfg(feature = "scheduler")]
pub use scheduler::*;

pub trait TestRunner: Default {
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
    fn run(self, assertion: Assertion<'_>) -> TestResult<'_, Self::Error>;
}
impl<E: core::error::Error> Test for for<'a> fn(Assertion<'a>) -> Result<(), E> {
    type Error = E;
    fn run(self, assertion: Assertion<'_>) -> TestResult<'_, Self::Error> {
        let test_name = assertion.test_name();
        if let Err(e) = self(assertion) {
            Err(TestError::new(test_name, e))
        } else {
            Ok(AssertionSuccessful::new(test_name))
        }
    }
}
