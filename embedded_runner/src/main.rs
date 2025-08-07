#![no_std]
#![no_main]

use cortex_m_rt::entry;
use cortex_m_semihosting::{debug, hprintln};
use embedded_tester::{
    TestRunner, TestScheduler,
    assertion::{Assertion, AssertionFailure},
};
use panic_halt as _;

#[entry]
fn main() -> ! {
    hprintln!("BEGAN");
    for result in TestSchedulerB::default().execute() {
        match result {
            Ok(assertion_success) => {
                hprintln!("SUCCESS: {}", assertion_success.test_name());
            }
            Err(e) => {
                hprintln!("FAILURE: {}", e);
            }
        }
    }
    hprintln!("ENDED");
    debug::exit(debug::EXIT_SUCCESS);
    loop {}
}

#[derive(TestScheduler)]
struct TestSchedulerA {
    tests_a: TestsA,
    tests_b: TestsB,
}

#[derive(TestScheduler)]
struct TestSchedulerB {
    test_s: TestSchedulerA,
    tests_c: TestsC,
}

const ERROR_DESCRIPTION_SIZE: usize = 32;
#[derive(TestRunner)]
struct TestsA {
    a: fn(Assertion) -> Result<(), AssertionFailure<ERROR_DESCRIPTION_SIZE>>,
    b: fn(Assertion) -> Result<(), CustomError>,
    c: fn(Assertion) -> Result<(), CustomError>,
}

#[derive(TestRunner)]
struct TestsB {
    a: fn(Assertion) -> Result<(), AssertionFailure<ERROR_DESCRIPTION_SIZE>>,
}

#[derive(TestRunner)]
struct TestsC {
    a: fn(Assertion) -> Result<(), AssertionFailure<ERROR_DESCRIPTION_SIZE>>,
}

impl Default for TestsA {
    fn default() -> Self {
        TestsA {
            a: test_a,
            b: test_b,
            c: test_b,
        }
    }
}

impl Default for TestsB {
    fn default() -> Self {
        TestsB { a: test_a }
    }
}

impl Default for TestsC {
    fn default() -> Self {
        TestsC { a: test_a }
    }
}

fn test_a(a: Assertion) -> Result<(), AssertionFailure<ERROR_DESCRIPTION_SIZE>> {
    a.assert_eq(1, 1)?;
    Ok(())
}

#[derive(Debug)]
enum CustomError {
    Error1,
}
impl core::fmt::Display for CustomError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            CustomError::Error1 => write!(f, "FIRST_ERROR"),
        }
    }
}
impl core::error::Error for CustomError {}
fn test_b(_a: Assertion) -> Result<(), CustomError> {
    Err(CustomError::Error1)
}
