#[cfg(all(not(test), not(feature = "heapless")))]
use alloc::{
    format,
    string::{String, ToString},
};
#[cfg(feature = "heapless")]
use heapless::String;
#[cfg(test)]
use std::{
    format,
    string::{String, ToString},
};

use crate::error::TestError;

pub struct Assertion<#[cfg(feature = "heapless")] 'a> {
    #[cfg(not(feature = "heapless"))]
    test_name: String,
    #[cfg(feature = "heapless")]
    test_name: &'a str,
}
#[cfg(not(feature = "heapless"))]
impl Assertion {
    pub(crate) fn new(test_name: String) -> Self {
        Assertion { test_name }
    }
    pub fn assert(
        &self,
        assertion: impl for<'a> FnOnce(AssertionResult<'a>) -> Result<AssertionSuccessful, TestError>,
    ) -> Result<(), TestError> {
        match (assertion)(AssertionResult {
            test_name: &self.test_name,
        }) {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }
    pub fn assert_eq<A: PartialEq<B> + core::fmt::Debug, B: core::fmt::Debug>(
        &self,
        a: A,
        b: B,
    ) -> Result<(), TestError> {
        self.assert(move |r| {
            if a == b {
                Ok(r.success())
            } else {
                Err(r.failure(format!(
                    "expected left side to equal right side, but: {a:?} != {b:?}"
                )))
            }
        })
    }
    pub fn assert_ne<A: PartialEq<B> + core::fmt::Debug, B: core::fmt::Debug>(
        &self,
        a: A,
        b: B,
    ) -> Result<(), TestError> {
        self.assert(move |r| {
            if a != b {
                Ok(r.success())
            } else {
                Err(r.failure(format!(
                    "expected left side to not equal right side, but: {a:?} == {b:?}"
                )))
            }
        })
    }
}

#[cfg(feature = "heapless")]
impl<'a> Assertion<'a> {
    pub(crate) fn new(test_name: &'a str) -> Self {
        Assertion { test_name }
    }
    pub fn assert(
        &self,
        assertion: impl for<'b> FnOnce(
            AssertionResult<'b>,
        ) -> Result<AssertionSuccessful, TestError<'a>>,
    ) -> Result<(), TestError<'a>> {
        match (assertion)(AssertionResult {
            test_name: &self.test_name,
        }) {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }
    /// `MESSAGE_SIZE`: known characters length is 50
    pub fn assert_eq<
        const MESSAGE_SIZE: usize,
        A: PartialEq<B> + core::fmt::Debug,
        B: core::fmt::Debug,
    >(
        &self,
        a: A,
        b: B,
    ) -> Result<(), TestError> {
        self.assert(move |r| {
            if a == b {
                Ok(r.success())
            } else {
                #[cfg(not(feature = "heapless"))]
                {
                    Err(r.failure(format!(
                        "expected left side to equal right side, but: {a:?} != {b:?}"
                    )))
                }
                #[cfg(feature = "heapless")]
                {
                    let mut message = String::<MESSAGE_SIZE>::new();
                    write!(
                        message,
                        "expected left side to equal right side, but: {a:?} != {b:?}",
                    );
                    Err(r.failure(&message))
                }
            }
        })
    }
    /// `MESSAGE_SIZE`: known characters length is 53
    pub fn assert_ne<
        const MESSAGE_SIZE: usize,
        A: PartialEq<B> + core::fmt::Debug,
        B: core::fmt::Debug,
    >(
        &self,
        a: A,
        b: B,
    ) -> Result<(), TestError> {
        self.assert(move |r| {
            if a != b {
                Ok(r.success())
            } else {
                #[cfg(not(feature = "heapless"))]
                {
                    Err(r.failure(format!(
                        "expected left side to not equal right side, but: {a:?} == {b:?}"
                    )))
                }
                #[cfg(feature = "heapless")]
                {
                    let message = String::<MESSAGE_SIZE>::new();
                    write!(
                        message,
                        "expected left side to not equal right side, but: {a:?} == {b:?}",
                    );
                }
            }
        })
    }
}

pub struct AssertionResult<'a> {
    test_name: &'a str,
}

impl<'a> AssertionResult<'a> {
    pub fn success(self) -> AssertionSuccessful {
        AssertionSuccessful
    }
    #[cfg(not(feature = "heapless"))]
    pub fn failure(self, description: String) -> TestError {
        TestError::new(self.test_name.to_string(), description)
    }
    #[cfg(feature = "heapless")]
    pub fn failure(self, description: String) -> TestError {
        TestError::new(self.test_name, description)
    }
}

pub struct AssertionSuccessful;

#[cfg(test)]
mod test {
    use crate::assertion::Assertion;

    #[test]
    fn assert_success() {
        let assertion = Assertion::new("TEST_NAME".into());
        assert!(assertion.assert(|a| Ok(a.success())).is_ok());
    }
    #[test]
    fn assert_failure() {
        let assertion = Assertion::new("TEST_NAME".into());
        assert!(
            assertion
                .assert(|a| Err(a.failure("TEST_FAILURE_DESCRIPTION".into())))
                .is_err()
        );
    }
    #[test]
    fn assert_eq() {
        let assertion = Assertion::new("TEST_NAME".into());
        assert!(assertion.assert_eq(1, 1).is_ok());
        assert!(assertion.assert_ne(1, 1).is_err());
    }
    #[test]
    fn assert_ne() {
        let assertion = Assertion::new("TEST_NAME".into());
        assert!(assertion.assert_ne(1, 2).is_ok());
        assert!(assertion.assert_eq(1, 2).is_err());
    }
    #[test]
    fn assert_test_name() {
        const TEST_NAME: &str = "TEST_NAME";
        let assertion = Assertion::new(TEST_NAME.into());
        assert_eq!(TEST_NAME, assertion.test_name);
    }
    #[test]
    fn assert_failure_test_name() {
        const TEST_NAME: &str = "TEST_NAME";
        let assertion = Assertion::new(TEST_NAME.into());
        let result = assertion.assert_eq(1, 2);
        assert!(result.is_err());
        let error = result.unwrap_err();
        assert_eq!(TEST_NAME, error.test_name());
    }
}
