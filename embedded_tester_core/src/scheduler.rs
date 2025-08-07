use crate::TestResult;

pub trait TestScheduler {
    fn execute(self, logger: impl TestLogger);
}

pub trait TestLogger {
    fn log(&self, message: &str);
    fn log_test<E: core::error::Error>(&self, test_result: TestResult<'_, E>);
}
