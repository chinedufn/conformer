use std::hash::Hash;

pub use self::test_case_result::*;

mod test_case_result;

/// The results of all of the test cases in the test suite that was run.
pub struct TestCaseResults<MetaKey: Hash + Eq, MetaValue> {
    results: Vec<TestCaseResult<MetaKey, MetaValue>>,
}

impl<MetaKey: Hash + Eq, MetaValue> TestCaseResults<MetaKey, MetaValue> {
    #[allow(missing_docs)]
    pub fn new(results: Vec<TestCaseResult<MetaKey, MetaValue>>) -> Self {
        TestCaseResults { results }
    }

    /// The results of running all of the test cases.
    pub fn results(&self) -> &Vec<TestCaseResult<MetaKey, MetaValue>> {
        &self.results
    }

    /// The owned results of running all of the test cases.
    pub fn results_owned(self) -> Vec<TestCaseResult<MetaKey, MetaValue>> {
        self.results
    }
}
