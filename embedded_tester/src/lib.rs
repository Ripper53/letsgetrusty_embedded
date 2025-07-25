#![no_std]

#[cfg(not(test))]
extern crate alloc;
#[cfg(test)]
extern crate std;

#[cfg(not(test))]
use alloc::{boxed::Box, format, string::String, vec::Vec};
#[cfg(test)]
use std::{boxed::Box, format, string::String, vec::Vec};

use crate::{assertion::Assertion, error::TestError};

pub mod assertion;
pub mod error;

pub struct TestRunner<
    #[cfg(feature = "heapless")] 'a,
    #[cfg(feature = "heapless")] const COUNT: usize,
> {
    #[cfg(not(feature = "heapless"))]
    tests: Vec<TestContext>,
    #[cfg(feature = "heapless")]
    tests: [TestContext<'a>; COUNT],
}
pub struct TestContext<#[cfg(feature = "heapless")] 'a> {
    #[cfg(not(feature = "heapless"))]
    name: String,
    #[cfg(feature = "heapless")]
    name: &'a str,
    test: Test,
}
#[cfg(not(feature = "heapless"))]
impl TestContext {
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn run(self) -> Result<(), TestError> {
        self.test.run(Assertion::new(self.name))
    }
}
#[cfg(feature = "heapless")]
impl<'a> TestContext<'a> {
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn run(self) -> Result<(), TestError<'a>> {
        self.test.run(Assertion::new(self.name))
    }
}
#[cfg(not(feature = "heapless"))]
pub struct Test(Box<dyn FnOnce(Assertion) -> Result<(), TestError>>);
#[cfg(feature = "heapless")]
pub struct Test<const DescriptionLength: usize>(
    for<'a> fn(Assertion) -> Result<(), TestError<'a, DescriptionLength>>,
);
impl Test {
    pub fn run(self, assertion: Assertion) -> Result<(), TestError> {
        (self.0)(assertion)
    }
}
impl<T: FnOnce(Assertion) -> Result<(), TestError> + 'static> From<T> for Test {
    fn from(value: T) -> Self {
        Test(Box::new(value))
    }
}
/*impl<T: FnOnce(Assertion) -> Result<(), TestError> + 'static> From<Box<T>> for Test {
    fn from(value: T) -> Self {
        Test(value)
    }
}*/

#[cfg(not(feature = "heapless"))]
impl TestRunner {
    pub fn new() -> Self {
        TestRunner { tests: Vec::new() }
    }
    pub fn add_test(&mut self, name: String, test: impl Into<Test>) {
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
    fn run(self) -> Result<(), TestErrors> {
        let total_tests = self.tests.len();
        let mut errors: Vec<_> = self
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

#[cfg(feature = "heapless")]
impl<'a, const COUNT: usize> TestRunner<'a, COUNT> {
    pub fn new(tests: [TestContext; COUNT]) -> Self {
        TestRunner { tests }
    }
    pub fn add_test(&mut self, name: String, test: impl Into<Test>) {
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
        let mut errors: Vec<_> = self
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

#[derive(Debug)]
pub struct TestErrors<
    #[cfg(feature = "heapless")] 'a,
    #[cfg(feature = "heapless")] const ErrorCount: usize,
> {
    total_tests: usize,
    #[cfg(not(feature = "heapless"))]
    errors: Vec<TestError>,
    #[cfg(feature = "heapless")]
    errors: [TestError<'a>; ErrorCount],
}

macro_rules! test_error_fmt_impl {
    ($self: expr, $f: expr) => {
        #[cfg(feature = "heapless")]
        let mut error_message = String::new();
        #[cfg(not(feature = "heapless"))]
        let mut error_message = String::new();
        for error in $self.errors.iter() {
            error_message.push_str(&format!("{}: {}", error.test_name(), error.description()));
            error_message.push('\n');
        }
        let error_count = $self.errors.len();
        error_message.push_str(&format!(
            "❌ {}/{} tests succeeded, {error_count} failures!",
            $self.total_tests - error_count,
            $self.total_tests,
        ));
        write!($f, "{}", error_message)
    };
}
#[cfg(not(feature = "heapless"))]
impl core::fmt::Display for TestErrors {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        test_error_fmt_impl!(self, f);
    }
}
#[cfg(not(feature = "heapless"))]
impl core::error::Error for TestErrors {}
#[cfg(feature = "heapless")]
impl<'a, const ErrorCount: usize> core::fmt::Display for TestErrors<'a, ErrorCount> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        test_error_fmt_impl!(self, f);
    }
}
#[cfg(feature = "heapless")]
impl<'a, const ErrorCount: usize> core::error::Error for TestErrors<'a, ErrorCount> {}
