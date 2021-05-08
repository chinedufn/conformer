use std::collections::HashMap;
use std::hash::Hash;

/// Whether or not the test case passed along with metadata that they results renderer can
/// make use of when rendering our the results of the conformance test.
pub struct TestCaseResult {
    title: String,
    description: String,
    did_pass: bool,
    metadata: HashMap<String, MetadataValue>,
}

/// The associated value for a metadata key.
pub enum MetadataValue {
    #[allow(missing_docs)]
    String(String),
    #[allow(missing_docs)]
    Bytes(Vec<u8>),
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
    pub fn insert_metadata(&mut self, key: String, value: MetadataValue) -> Option<MetadataValue> {
        self.metadata.insert(key, value)
    }

    /// Describes why the test passed or failed.
    pub fn did_pass(&self) -> bool {
        self.did_pass
    }

    /// Information that the test runner stored about the test case.
    /// Used by test result processors when deciding what to do with the test results.
    pub fn metadata(&self) -> &HashMap<String, MetadataValue> {
        &self.metadata
    }
}
