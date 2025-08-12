use core::fmt::{Arguments, Write};

use heapless::String;

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
        const ERROR_DESCRIPTION_LENGTH: usize,
        A: PartialEq<B> + core::fmt::Display,
        B: core::fmt::Display,
    >(
        &self,
        a: A,
        b: B,
    ) -> Result<(), AssertionFailure<ERROR_DESCRIPTION_LENGTH>> {
        self.assert(move || {
            if a == b {
                Ok(())
            } else {
                Err(AssertionFailure::new(format_args!("expected {a} == {b}",)))
            }
        })
    }
    #[must_use]
    pub fn assert_ne<
        const ERROR_DESCRIPTION_LENGTH: usize,
        A: PartialEq<B> + core::fmt::Display,
        B: core::fmt::Display,
    >(
        &self,
        a: A,
        b: B,
    ) -> Result<(), AssertionFailure<ERROR_DESCRIPTION_LENGTH>> {
        self.assert(move || {
            if a != b {
                Ok(())
            } else {
                Err(AssertionFailure::new(format_args!("expected {a} != {b}",)))
            }
        })
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
/// `ERROR_DESCRIPTION_LENGTH` is the max length of the failure description this can hold.
/// Longer strings will be truncated.
#[derive(Debug)]
pub struct AssertionFailure<const ERROR_DESCRIPTION_LENGTH: usize> {
    description: String<ERROR_DESCRIPTION_LENGTH>,
}
impl<const ERROR_DESCRIPTION_LENGTH: usize> core::fmt::Display
    for AssertionFailure<ERROR_DESCRIPTION_LENGTH>
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.description)
    }
}
impl<const ERROR_DESCRIPTION_LENGTH: usize> core::error::Error
    for AssertionFailure<ERROR_DESCRIPTION_LENGTH>
{
}
impl<const ERROR_DESCRIPTION_LENGTH: usize> AssertionFailure<ERROR_DESCRIPTION_LENGTH> {
    pub fn new(description: Arguments) -> AssertionFailure<ERROR_DESCRIPTION_LENGTH> {
        let mut description_str = String::<ERROR_DESCRIPTION_LENGTH>::new();
        // Ignore error when appending message too long for the allocated memory.
        let _ = TruncateWriter(&mut description_str).write_fmt(description);
        AssertionFailure {
            description: description_str,
        }
    }
    pub fn error_description(&self) -> &str {
        self.description.as_str()
    }
    pub fn take_error_description(self) -> String<ERROR_DESCRIPTION_LENGTH> {
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
    use crate::assertion::{Assertion, AssertionFailure};

    #[test]
    fn assert_success() {
        #[derive(Debug)]
        struct TestError;
        impl core::fmt::Display for TestError {
            fn fmt(&self, _: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
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
            Err(AssertionFailure::<ERROR_MESSAGE_LEN>::new(format_args!(
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
