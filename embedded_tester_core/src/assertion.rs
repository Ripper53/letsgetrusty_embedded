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
    pub fn assert<E: core::error::Error + 'a>(
        &'a self,
        assertion: impl FnOnce(AssertionResult<'a>) -> Result<(), E>,
    ) -> Result<(), E> {
        match assertion(AssertionResult { assertion: self }) {
            Ok(_) => Ok(()),
            Err(assertion_failure) => Err(assertion_failure),
        }
    }
    pub fn assert_eq<
        const ErrorDescriptionLength: usize,
        A: PartialEq<B> + core::fmt::Debug,
        B: core::fmt::Debug,
    >(
        &self,
        a: A,
        b: B,
    ) -> Result<(), AssertionFailure<ErrorDescriptionLength>> {
        self.assert(move |r| {
            if a == b {
                Ok(())
            } else {
                Err(r.failure(format_args!(
                    "expected left side to equal right side, but: {a:?} != {b:?}",
                )))
            }
        })
    }
    pub fn assert_ne<
        const ErrorDescriptionLength: usize,
        A: PartialEq<B> + core::fmt::Debug,
        B: core::fmt::Debug,
    >(
        &self,
        a: A,
        b: B,
    ) -> Result<(), AssertionFailure<ErrorDescriptionLength>> {
        self.assert(move |r| {
            if a != b {
                Ok(())
            } else {
                Err(r.failure(format_args!(
                    "expected left side to not equal right side, but: {a:?} == {b:?}",
                )))
            }
        })
    }
}

pub struct AssertionResult<'a> {
    assertion: &'a Assertion<'a>,
}

impl<'a> AssertionResult<'a> {
    pub fn success(self) -> AssertionSuccessful<'a> {
        AssertionSuccessful::new(self.assertion.test_name)
    }
    pub fn failure<const ErrorDescriptionLength: usize>(
        mut self,
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

    use crate::{assertion::Assertion, error::TestError};

    #[test]
    fn assert_success() {
        let assertion = new_assertion::<256>("TEST_NAME");
        assert!(assertion.assert(|a| Ok(a.success())).is_ok());
    }
    #[test]
    fn assert_failure() {
        const ERROR_MESSAGE: &str = "TEST_FAILURE_DESCRIPTION";
        const ERROR_MESSAGE_LEN: usize = ERROR_MESSAGE.len();
        let assertion = new_assertion::<ERROR_MESSAGE_LEN>("TEST_NAME");
        let assertion = assertion.assert(|a| Err(a.failure(format_args!("{ERROR_MESSAGE}"))));
        assert!(assertion.is_err());
        let error = assertion.unwrap_err();
        assert_eq!(ERROR_MESSAGE_LEN, error.description().len());
        assert_eq!(ERROR_MESSAGE, error.description());
    }
    #[test]
    fn assert_eq() {
        let assertion = new_assertion::<256>("TEST_NAME");
        let assertion = assertion.assert_eq(1, 1);
        assert!(assertion.is_ok());
        let assertion = assertion.unwrap();
        assert!(assertion.0.assert_ne(1, 1).is_err());
    }
    #[test]
    fn assert_ne() {
        let assertion = new_assertion::<256>("TEST_NAME");
        let assertion = assertion.assert_ne(1, 2);
        assert!(assertion.is_ok());
        let assertion = assertion.unwrap();
        assert!(assertion.0.assert_eq(1, 2).is_err());
    }
    #[test]
    fn assert_test_name() {
        const TEST_NAME: &str = "TEST_NAME";
        let assertion = new_assertion::<256>(TEST_NAME);
        assert_eq!(TEST_NAME, assertion.test_error.test_name());
    }
    #[test]
    fn assert_failure_test_name() {
        const TEST_NAME: &str = "TEST_NAME";
        let assertion = new_assertion::<256>(TEST_NAME);
        let result = assertion.assert_eq(1, 2);
        assert!(result.is_err());
        let error = result.unwrap_err();
        assert_eq!(TEST_NAME, error.test_name());
    }

    fn new_assertion<const ErrorDescriptionLength: usize>(
        test_name: &str,
    ) -> Assertion<'_, ErrorDescriptionLength> {
        let test_error = TestError::new(test_name, String::<ErrorDescriptionLength>::new());
        Assertion::new(test_error)
    }
}
