//! A framework for authors of conformance test suites.

#![deny(missing_docs)]

use std::hash::Hash;

pub use self::test_case_results::*;

mod test_case_results;

/// Run a test suite.
pub fn run_test_suite<TypeToTest, TypeToTestCreator, MetaKey: Hash + Eq, MetaValue>(
    type_to_test_creator: TypeToTestCreator,
    suite: Vec<TestCase<TypeToTest, MetaKey, MetaValue>>,
) -> TestCaseResults<MetaKey, MetaValue>
where
    TypeToTestCreator: Fn(&dyn TestCase<TypeToTest, MetaKey, MetaValue>) -> TypeToTest,
{
    let mut results = Vec::with_capacity(suite.len());

    for test_case in suite {
        let type_to_test = type_to_test_creator(&test_case);

        let test_case_result = test_case.run(type_to_test);
        results.push(test_case_result);
    }

    TestCaseResults::new(results)
}

/// Used to indicate a type that can be used to create test cases to test some aspect of a
/// Renderer.
pub trait TestCase<TypeToTest, MetaKey: Hash + Eq, MetaValue> {
    /// A title for the test case.
    fn title(&self) -> String;

    /// A description of the test case.
    fn description(&self) -> String;

    /// Ok if the test case was successful, otherwise an error message is returned.
    fn run(self, type_to_test: TypeToTest) -> TestCaseResult<MetaKey, MetaValue>;
}
