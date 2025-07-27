use core::fmt::{Arguments, Write};

use heapless::String;

use crate::error::TestError;

#[derive(Debug)]
pub struct Assertion<'a> {
    test_name: &'a str,
}

impl<'a> Assertion<'a> {
    pub fn new(test_name: &'a str) -> Self {
        Assertion { test_name }
    }
    pub fn test_name(&self) -> &'a str {
        self.test_name
    }
    #[must_use]
    pub fn assert<E: core::error::Error + 'a>(
        &'a self,
        assertion: impl FnOnce() -> Result<(), E>,
    ) -> Result<(), E> {
        match assertion() {
            Ok(_) => Ok(()),
            Err(assertion_failure) => Err(assertion_failure),
        }
    }
    #[must_use]
    pub fn assert_eq<
        const ErrorDescriptionLength: usize,
        A: PartialEq<B> + core::fmt::Display,
        B: core::fmt::Display,
    >(
        &self,
        a: A,
        b: B,
    ) -> Result<(), AssertionFailure<ErrorDescriptionLength>> {
        self.assert(move || {
            if a == b {
                Ok(())
            } else {
                Err(AssertionResult::failure(format_args!(
                    "expected left side to equal right side, but: {a} != {b}",
                )))
            }
        })
    }
    #[must_use]
    pub fn assert_ne<
        const ErrorDescriptionLength: usize,
        A: PartialEq<B> + core::fmt::Display,
        B: core::fmt::Display,
    >(
        &self,
        a: A,
        b: B,
    ) -> Result<(), AssertionFailure<ErrorDescriptionLength>> {
        self.assert(move || {
            if a != b {
                Ok(())
            } else {
                Err(AssertionResult::failure(format_args!(
                    "expected left side to not equal right side, but: {a} == {b}",
                )))
            }
        })
    }
}

pub struct AssertionResult;

impl AssertionResult {
    pub fn failure<const ErrorDescriptionLength: usize>(
        description: Arguments,
    ) -> AssertionFailure<ErrorDescriptionLength> {
        let mut description_str = String::<ErrorDescriptionLength>::new();
        // Ignore error when appending message too long for the allocated memory.
        let _ = TruncateWriter(&mut description_str).write_fmt(description);
        AssertionFailure {
            description: description_str,
        }
    }
}

#[derive(Debug)]
pub struct AssertionSuccessful<'a> {
    test_name: &'a str,
}
impl<'a> AssertionSuccessful<'a> {
    pub(crate) fn new(test_name: &'a str) -> Self {
        AssertionSuccessful { test_name }
    }
    pub fn test_name(&self) -> &str {
        self.test_name
    }
}
#[derive(Debug)]
pub struct AssertionFailure<const ErrorDescriptionLength: usize> {
    description: String<ErrorDescriptionLength>,
}
impl<const ErrorDescriptionLength: usize> core::fmt::Display
    for AssertionFailure<ErrorDescriptionLength>
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.description)
    }
}
impl<const ErrorDescriptionLength: usize> core::error::Error
    for AssertionFailure<ErrorDescriptionLength>
{
}
impl<const ErrorDescriptionLength: usize> AssertionFailure<ErrorDescriptionLength> {
    pub fn error_description(&self) -> &str {
        self.description.as_str()
    }
    pub fn take_error_description(self) -> String<ErrorDescriptionLength> {
        self.description
    }
}

struct TruncateWriter<'a, const N: usize>(&'a mut String<N>);
impl<'a, const N: usize> Write for TruncateWriter<'a, N> {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        let truncated = if s.len() > N { &s[..N] } else { s };
        self.0.push_str(truncated).map_err(|()| core::fmt::Error)
    }
}

#[cfg(test)]
mod test {
    use heapless::String;

    use crate::{
        assertion::{Assertion, AssertionResult},
        error::TestError,
    };

    #[test]
    fn assert_success() {
        #[derive(Debug)]
        struct TestError;
        impl core::fmt::Display for TestError {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                unimplemented!()
            }
        }
        impl core::error::Error for TestError {}
        let assertion = Assertion::new("TEST_NAME");
        assert!(assertion.assert::<TestError>(|| Ok(())).is_ok());
    }
    #[test]
    fn assert_failure() {
        const ERROR_MESSAGE: &str = "TEST_FAILURE_DESCRIPTION";
        const ERROR_MESSAGE_LEN: usize = ERROR_MESSAGE.len();
        let assertion = Assertion::new("TEST_NAME");
        let assertion = assertion.assert(|| {
            Err(AssertionResult::failure::<ERROR_MESSAGE_LEN>(format_args!(
                "{ERROR_MESSAGE}"
            )))
        });
        assert!(assertion.is_err());
        let error = assertion.unwrap_err();
        assert_eq!(ERROR_MESSAGE_LEN, error.error_description().len());
        assert_eq!(ERROR_MESSAGE, error.error_description());
    }
    #[test]
    fn assert_eq() {
        let assertion = Assertion::new("TEST_NAME");
        assert!(assertion.assert_eq::<0, _, _>(1, 1).is_ok());
        assert!(assertion.assert_ne::<0, _, _>(1, 1).is_err());
    }
    #[test]
    fn assert_ne() {
        let assertion = Assertion::new("TEST_NAME");
        assert!(assertion.assert_ne::<0, _, _>(1, 2).is_ok());
        assert!(assertion.assert_eq::<0, _, _>(1, 2).is_err());
    }
    #[test]
    fn assert_test_name() {
        const TEST_NAME: &str = "TEST_NAME";
        let assertion = Assertion::new(TEST_NAME);
        assert_eq!(TEST_NAME, assertion.test_name());
    }
}
