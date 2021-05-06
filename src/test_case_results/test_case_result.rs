use std::collections::HashMap;
use std::hash::Hash;

/// Whether or not the test case passed along with metadata that they results renderer can
/// make use of when rendering our the results of the conformance test.
pub struct TestCaseResult<MetaKey: Hash + Eq, MetaValue> {
    passed_or_failed: PassedOrFailed,
    metadata: HashMap<MetaKey, MetaValue>,
}

/// Whether the test case passed or failed.
pub enum PassedOrFailed {
    /// The reason that the test case passed.
    Passed(String),
    /// The reason that the test case failed.
    Failed(String),
}

impl<MetaKey: Hash + Eq, MetaValue> TestCaseResult<MetaKey, MetaValue> {
    #[allow(missing_docs)]
    pub fn new(passed_or_failed: PassedOrFailed) -> Self {
        ConformanceTestCaseResult {
            passed_or_failed,
            metadata: HashMap::new(),
        }
    }

    /// Insert metadata for the test case result.
    pub fn insert_metadata(&mut self, key: MetaKey, value: MetaValue) -> Option<MetaValue> {
        self.metadata.insert(key, value)
    }

    /// Describes why the test passed or failed.
    pub fn passed_or_failed(&self) -> &PassedOrFailed {
        &self.passed_or_failed
    }

    /// Information that the test runner stored about the test case.
    /// Used by test result processors when deciding what to do with the test results.
    pub fn metadata(&self) -> &HashMap<MetaKey, MetaValue> {
        &self.metadata
    }
}
