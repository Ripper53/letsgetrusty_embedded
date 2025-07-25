#![no_std]
#![no_main]

use cortex_m_rt::entry;
use cortex_m_semihosting::{debug, hprintln};
use embedded_tester::{
    Test, TestResult, TestRunner,
    assertion::{Assertion, AssertionSuccessful},
    error::TestError,
};
use panic_halt as _;

#[entry]
fn main() -> ! {
    hprintln!("BEGAN");

    debug::exit(debug::EXIT_SUCCESS);
    loop {}
}

#[derive(TestRunner)]
#[test_runner(error_message_size = 256)]
struct Tests<const ErrorDescriptionLength: usize> {
    a: Test<ErrorDescriptionLength>,
}

const ERROR_DESCRIPTION_SIZE: usize = 256;
fn test_a<'a>(a: Assertion<'a, ERROR_DESCRIPTION_SIZE>) -> TestResult<'a, ERROR_DESCRIPTION_SIZE> {
    let a = a.assert_eq(1, 1)?;
    Ok(())
}

struct Iter<'a, const ERROR_DESCRIPTION_SIZE: usize> {
    index: usize,
}
impl<'a, const ERROR_DESCRIPTION_SIZE: usize> ::core::iter::Iterator
    for Iter<'a, ERROR_DESCRIPTION_SIZE>
{
    type Item = fn(
        ::embedded_tester::Assertion<'a, ERROR_DESCRIPTION_SIZE>,
    ) -> TestResult<'a, ERROR_DESCRIPTION_SIZE>;
    fn next(&mut self) -> ::core::option::Option<Self::Item> {
        self.index += 1;
        match self.index {
            0 => self.a,
        }
    }
}
