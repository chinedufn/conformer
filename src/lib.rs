//! A simple framework for authoring conformance test suites.

#![deny(missing_docs)]

use std::hash::Hash;

pub use self::test_case_results::*;
pub use self::test_suite::*;
pub use self::visualizer::*;

mod test_case_results;
mod test_suite;
mod visualizer;

/// Run a test suite.
pub fn run_test_suite<TypeToTest, TypeToTestCreator>(
    type_to_test_creator: TypeToTestCreator,
    suite: TestSuite<TypeToTest>,
) -> TestCaseResults
where
    TypeToTestCreator: Fn(&Box<dyn TestCase<TypeToTest>>) -> TypeToTest,
{
    let mut results = Vec::with_capacity(suite.test_cases.len());

    for test_case in suite.test_cases {
        let type_to_test = type_to_test_creator(&test_case);

        let test_case_result = test_case.run(type_to_test);
        results.push(test_case_result);
    }

    TestCaseResults::new(suite.title, suite.description, results)
}

/// Used to indicate a type that can be used to create test cases to test some aspect of a
/// Renderer.
pub trait TestCase<TypeToTest> {
    /// Ok if the test case was successful, otherwise an error message is returned.
    fn run(&self, type_to_test: TypeToTest) -> TestCaseResult;
}
