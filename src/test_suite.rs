use crate::TestCase;
use std::hash::Hash;

/// A test suite to run.
pub struct TestSuite<TypeToTest> {
    /// The test suite's title
    pub title: String,
    /// The test suite's description
    pub description: String,
    /// The test cases
    pub test_cases: Vec<Box<dyn TestCase<TypeToTest>>>,
}

impl<TypeToTest> TestSuite<TypeToTest> {
    /// Create a new test suite.
    pub fn new(
        title: String,
        description: String,
        test_cases: Vec<Box<dyn TestCase<TypeToTest>>>,
    ) -> Self {
        TestSuite {
            title,
            description,
            test_cases,
        }
    }
}
