use crate::TestCaseResults;

/// A basic list of passing and failing tests, useful for visualizing in a terminal.
pub struct ViewSimpleText {}

impl ViewSimpleText {
    #[allow(missing_docs)]
    pub fn new() -> Self {
        ViewSimpleText {}
    }

    /// Prints the titles an descriptions of tests that passed and failed.
    pub fn process_test_results(&self, test_results: &TestCaseResults) -> String {
        let result_or_results = if test_results.results().len() == 1 {
            "result"
        } else {
            "results"
        };

        let mut output = format!(
            r#"{title}
{description}

{result_count} test {result_or_results}
"#,
            title = test_results.suite_title(),
            description = test_results.suite_description(),
            result_count = test_results.results().len(),
            result_or_results = result_or_results
        );

        let mut pass_count = 0;
        let mut fail_count = 0;

        for test_result in test_results.results() {
            let ok_or_failed = if test_result.did_pass() {
                pass_count += 1;
                "ok"
            } else {
                fail_count += 1;
                "FAILED"
            };

            output += &format!(
                r#"{title} ... {ok_or_failed}
"#,
                title = test_result.title(),
                ok_or_failed = ok_or_failed
            );
        }

        let pass_or_fail = if fail_count == 0 { "ok" } else { "FAILED" };

        output += &format!(
            r#"
test result: {pass_or_fail}. {pass_count} passed; {fail_count} failed"#,
            pass_or_fail = pass_or_fail,
            pass_count = pass_count,
            fail_count = fail_count
        );

        output
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::TestCaseResult;

    /// Verify the output of one passing test case.
    #[test]
    fn one_test_case_passed() {
        let test_results = test_suite_one(PassedOrFailed::Passed);
        let results = ViewSimpleText::new().process_test_results(&test_results);

        let expected = r#"My Test Suite Title
My Test Suite description.

1 test result
Test Case Title ... ok

test result: ok. 1 passed; 0 failed"#;

        assert_eq!(results, expected)
    }

    /// Verify the output of one failing test case.
    #[test]
    fn one_test_case_failed() {
        let test_results = test_suite_one(PassedOrFailed::Failed);
        let results = ViewSimpleText::new().process_test_results(&test_results);

        let expected = r#"My Test Suite Title
My Test Suite description.

1 test result
Test Case Title ... FAILED

test result: FAILED. 0 passed; 1 failed"#;

        assert_eq!(results, expected)
    }

    /// Verify the output of one passing and one failing test case.
    #[test]
    fn two_test_cases_one_pass_one_fail() {
        let test_results = test_suite_two(PassedOrFailed::Passed, PassedOrFailed::Failed);
        let results = ViewSimpleText::new().process_test_results(&test_results);

        let expected = r#"My Test Suite Title
My Test Suite description.

2 test results
Test Case Title ... ok
Test Case Title ... FAILED

test result: FAILED. 1 passed; 1 failed"#;

        assert_eq!(results, expected)
    }

    /// Verify the output of two passing test cases.
    #[test]
    fn two_test_cases_two_passing() {
        let test_results = test_suite_two(PassedOrFailed::Passed, PassedOrFailed::Passed);
        let results = ViewSimpleText::new().process_test_results(&test_results);

        let expected = r#"My Test Suite Title
My Test Suite description.

2 test results
Test Case Title ... ok
Test Case Title ... ok

test result: ok. 2 passed; 0 failed"#;

        assert_eq!(results, expected)
    }

    fn test_suite_one(pass_fail: PassedOrFailed) -> TestCaseResults {
        let result = TestCaseResult::new(
            test_case_title(),
            test_case_description(),
            pass_fail.did_pass(),
        );

        TestCaseResults::new(test_suite_title(), test_suite_description(), vec![result])
    }

    fn test_suite_two(pass_fail_1: PassedOrFailed, pass_fail_2: PassedOrFailed) -> TestCaseResults {
        let result1 = TestCaseResult::new(
            test_case_title(),
            test_case_description(),
            pass_fail_1.did_pass(),
        );
        let result2 = TestCaseResult::new(
            test_case_title(),
            test_case_description(),
            pass_fail_2.did_pass(),
        );

        TestCaseResults::new(
            test_suite_title(),
            test_suite_description(),
            vec![result1, result2],
        )
    }

    fn test_suite_title() -> String {
        "My Test Suite Title".to_string()
    }

    fn test_suite_description() -> String {
        "My Test Suite description.".to_string()
    }

    fn test_case_title() -> String {
        "Test Case Title".to_string()
    }

    fn test_case_description() -> String {
        "Test Case Description".to_string()
    }

    enum PassedOrFailed {
        Passed,
        Failed,
    }

    impl PassedOrFailed {
        fn did_pass(&self) -> bool {
            matches!(self, PassedOrFailed::Passed)
        }
    }
}
