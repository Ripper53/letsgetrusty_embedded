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

const ERROR_DESCRIPTION_SIZE: usize = 256;
#[derive(TestRunner)]
#[test_runner_config(error_message_size = 256)]
struct Tests {
    a: fn(Assertion) -> Result<(), AssertionFailure<ERROR_DESCRIPTION_SIZE>>,
}

impl Default for Tests {
    fn default() -> Self {
        Tests { a: test_a }
    }
}

fn test_a(a: Assertion) -> Result<(), AssertionFailure<ERROR_DESCRIPTION_SIZE>> {
    a.assert_eq(1, 2)?;
    Ok(())
}
