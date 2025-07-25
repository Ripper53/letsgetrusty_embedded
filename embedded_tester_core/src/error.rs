use heapless::String;

#[derive(Debug)]
pub struct TestError<'a, const DescriptionLength: usize> {
    test_name: &'a str,
    description: String<DescriptionLength>,
}
impl<'a, const DescriptionLength: usize> TestError<'a, DescriptionLength> {
    pub fn new(test_name: &'a str, description: String<DescriptionLength>) -> Self {
        TestError {
            test_name,
            description,
        }
    }
    pub fn test_name(&self) -> &str {
        self.test_name
    }
    pub fn description(&self) -> &str {
        self.description.as_str()
    }
    pub fn set_test_name(&mut self, test_name: &'a str) {
        self.test_name = test_name;
    }
    pub fn description_mut(&mut self) -> &mut String<DescriptionLength> {
        &mut self.description
    }
}

impl<'a, const ErrorDescriptionLength: usize> core::fmt::Display
    for TestError<'a, ErrorDescriptionLength>
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.description)
    }
}
impl<'a, const ErrorDescriptionLength: usize> core::error::Error
    for TestError<'a, ErrorDescriptionLength>
{
}
