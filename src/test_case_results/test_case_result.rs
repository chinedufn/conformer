use std::collections::HashMap;

/// Whether or not the test case passed along with metadata that they results renderer can
/// make use of when rendering our the results of the conformance test.
#[derive(Debug)]
pub struct TestCaseResult {
    title: String,
    description: String,
    did_pass: bool,
    metadata: HashMap<String, String>,
}

impl TestCaseResult {
    #[allow(missing_docs)]
    pub fn new(title: String, description: String, did_pass: bool) -> Self {
        TestCaseResult {
            title,
            description,
            did_pass,
            metadata: HashMap::new(),
        }
    }

    /// The test case's title.
    pub fn title(&self) -> &str {
        &self.title
    }

    /// The test case's description.
    pub fn description(&self) -> &str {
        &self.description
    }

    /// Insert metadata for the test case result.
    pub fn insert_metadata(&mut self, key: String, value: String) -> Option<String> {
        self.metadata.insert(key, value)
    }

    /// Describes why the test passed or failed.
    pub fn did_pass(&self) -> bool {
        self.did_pass
    }

    /// Information that the test runner stored about the test case.
    /// Used by test result processors when deciding what to do with the test results.
    pub fn metadata(&self) -> &HashMap<String, String> {
        &self.metadata
    }
}
