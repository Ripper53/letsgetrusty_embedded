#[cfg(not(feature = "heapless"))]
use alloc::string::String;
#[cfg(feature = "heapless")]
use heapless::String;

#[derive(Debug)]
pub struct TestError<#[cfg(feature = "heapless")] 'a, const DescriptionLength: usize> {
    #[cfg(not(feature = "heapless"))]
    test_name: String,
    #[cfg(feature = "heapless")]
    test_name: &'a str,
    #[cfg(not(feature = "heapless"))]
    description: String,
    #[cfg(feature = "heapless")]
    description: String<DescriptionLength>,
}
#[cfg(not(feature = "heapless"))]
impl TestError {
    pub(crate) fn new(test_name: String, description: String) -> Self {
        TestError {
            test_name,
            description,
        }
    }
    pub fn test_name(&self) -> &str {
        &self.test_name
    }
    pub fn description(&self) -> &str {
        &self.description
    }
}
#[cfg(feature = "heapless")]
impl<'a> TestError<'a> {
    pub(crate) fn new(test_name: &'a str, description: &'a str) -> Self {
        TestError {
            test_name,
            description,
        }
    }
    pub fn test_name(&self) -> &str {
        self.test_name
    }
    pub fn description(&self) -> &str {
        self.description
    }
}

#[cfg(not(feature = "heapless"))]
impl core::fmt::Display for TestError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.description)
    }
}
#[cfg(not(feature = "heapless"))]
impl core::error::Error for TestError {}
#[cfg(feature = "heapless")]
impl<'a> core::fmt::Display for TestError<'a> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.description)
    }
}
#[cfg(feature = "heapless")]
impl<'a> core::error::Error for TestError<'a> {}
