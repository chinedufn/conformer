pub use self::test_case_result::*;

mod test_case_result;

/// The results of all of the test cases in the test suite that was run.
#[derive(Debug)]
pub struct TestCaseResults {
    suite_title: String,
    suite_description: String,
    results: Vec<TestCaseResult>,
}

impl TestCaseResults {
    /// # Panics
    ///
    /// Panics if there are no test case results in order to catch cases where one accidentally
    /// filters out all tests.
    pub fn new(
        suite_title: String,
        suite_description: String,
        results: Vec<TestCaseResult>,
    ) -> Self {
        assert!(results.len() > 0);

        TestCaseResults {
            suite_title,
            suite_description,
            results,
        }
    }

    /// The test suite's title.
    pub fn suite_title(&self) -> &String {
        &self.suite_title
    }

    /// The test suite's description.
    pub fn suite_description(&self) -> &String {
        &self.suite_description
    }

    /// The results of running all of the test cases.
    pub fn results(&self) -> &Vec<TestCaseResult> {
        &self.results
    }

    /// Whether or not all of the test cases passed.
    pub fn did_pass(&self) -> bool {
        self.results.iter().all(|r| r.did_pass())
    }

    /// # Panics
    ///
    /// Panics if one or more test cases failed.
    pub fn assert_did_pass(&self) {
        assert!(self.did_pass());
    }
}
