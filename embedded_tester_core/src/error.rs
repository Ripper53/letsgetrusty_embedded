#[derive(Debug)]
pub struct TestError<'a, E: core::error::Error> {
    test_name: &'a str,
    error: E,
}
impl<'a, E: core::error::Error> TestError<'a, E> {
    pub(crate) fn new(test_name: &'a str, error: E) -> Self {
        TestError { test_name, error }
    }
    pub fn test_name(&self) -> &str {
        self.test_name
    }
    pub fn error(&self) -> &E {
        &self.error
    }
    pub fn map_error<NewError: core::error::Error>(
        mut self,
        f: impl FnOnce(E) -> NewError,
    ) -> TestError<'a, NewError> {
        TestError {
            test_name: self.test_name,
            error: f(self.error),
        }
    }
}

impl<'a, E: core::error::Error> core::fmt::Display for TestError<'a, E> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "failed test {}: {}", self.test_name, self.error)
    }
}
impl<'a, E: core::error::Error> core::error::Error for TestError<'a, E> {}
