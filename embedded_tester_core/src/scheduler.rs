use crate::TestResult;

pub trait TestScheduler {
    fn execute_suites(self, logger: impl TestLogger);
}

pub trait TestLogger {
    fn log_test_suite_name(&self, message: &str);
    fn log_test_result<E: core::error::Error>(&self, test_result: TestResult<'_, E>);
}
