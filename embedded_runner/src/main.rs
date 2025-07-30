#![no_std]
#![no_main]

use cortex_m_rt::entry;
use cortex_m_semihosting::{debug, hprintln};
use embedded_tester::{
    TestRunner,
    assertion::{Assertion, AssertionFailure},
};
use panic_halt as _;

#[entry]
fn main() -> ! {
    hprintln!("BEGAN");
    for result in Tests::default().execute() {
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

const ERROR_DESCRIPTION_SIZE: usize = 32;
#[derive(TestRunner)]
struct Tests {
    a: fn(Assertion) -> Result<(), AssertionFailure<ERROR_DESCRIPTION_SIZE>>,
    b: fn(Assertion) -> Result<(), CustomError>,
    c: fn(Assertion) -> Result<(), CustomError>,
}

impl Default for Tests {
    fn default() -> Self {
        Tests {
            a: test_a,
            b: test_b,
            c: test_b,
        }
    }
}

fn test_a(a: Assertion) -> Result<(), AssertionFailure<ERROR_DESCRIPTION_SIZE>> {
    a.assert_eq(1, 2)?;
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
