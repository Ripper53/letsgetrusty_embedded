use core::fmt::{Arguments, Write};

use heapless::String;

use crate::error::TestError;

#[derive(Debug)]
pub struct Assertion<'a, const ErrorDescriptionLength: usize> {
    test_error: TestError<'a, ErrorDescriptionLength>,
}

impl<'a, const ErrorDescriptionLength: usize> Assertion<'a, ErrorDescriptionLength> {
    pub fn new(test_error: TestError<'a, ErrorDescriptionLength>) -> Self {
        Assertion { test_error }
    }
    pub fn assert(
        mut self,
        assertion: impl for<'b> FnOnce(
            AssertionResult<'b, ErrorDescriptionLength>,
        ) -> Result<
            AssertionSuccessful<'b, ErrorDescriptionLength>,
            TestError<'b, ErrorDescriptionLength>,
        >,
    ) -> Result<
        AssertionSuccessful<'a, ErrorDescriptionLength>,
        TestError<'a, ErrorDescriptionLength>,
    > {
        assertion(AssertionResult { assertion: self })
    }
    pub fn assert_eq<A: PartialEq<B> + core::fmt::Debug, B: core::fmt::Debug>(
        self,
        a: A,
        b: B,
    ) -> Result<
        AssertionSuccessful<'a, ErrorDescriptionLength>,
        TestError<'a, ErrorDescriptionLength>,
    > {
        self.assert(move |r| {
            if a == b {
                Ok(r.success())
            } else {
                let mut message = String::<ErrorDescriptionLength>::new();
                Err(r.failure(format_args!(
                    "expected left side to equal right side, but: {a:?} != {b:?}",
                )))
            }
        })
    }
    pub fn assert_ne<A: PartialEq<B> + core::fmt::Debug, B: core::fmt::Debug>(
        self,
        a: A,
        b: B,
    ) -> Result<
        AssertionSuccessful<'a, ErrorDescriptionLength>,
        TestError<'a, ErrorDescriptionLength>,
    > {
        self.assert(move |r| {
            if a != b {
                Ok(r.success())
            } else {
                Err(r.failure(format_args!(
                    "expected left side to not equal right side, but: {a:?} == {b:?}",
                )))
            }
        })
    }
}

pub struct AssertionResult<'a, const ErrorDescriptionLength: usize> {
    assertion: Assertion<'a, ErrorDescriptionLength>,
}

impl<'a, const ErrorDescriptionLength: usize> AssertionResult<'a, ErrorDescriptionLength> {
    pub fn success(self) -> AssertionSuccessful<'a, ErrorDescriptionLength> {
        AssertionSuccessful(self.assertion)
    }
    pub fn failure(mut self, description: Arguments) -> TestError<'a, ErrorDescriptionLength> {
        let previous_description = self.assertion.test_error.description_mut();
        // Ignore error when appending message too long for the allocated memory.
        let _ = TruncateWriter(previous_description).write_fmt(description);
        self.assertion.test_error
    }
}

struct TruncateWriter<'a, const N: usize>(&'a mut String<N>);
impl<'a, const N: usize> Write for TruncateWriter<'a, N> {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        let truncated = if s.len() > N { &s[..N] } else { s };
        self.0.push_str(truncated).map_err(|()| core::fmt::Error)
    }
}

#[derive(Debug)]
pub struct AssertionSuccessful<'a, const ErrorDescriptionLength: usize>(
    Assertion<'a, ErrorDescriptionLength>,
);
impl<'a, const ErrorDescriptionLength: usize> AssertionSuccessful<'a, ErrorDescriptionLength> {
    pub fn test_name(&self) -> &str {
        self.0.test_error.test_name()
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
