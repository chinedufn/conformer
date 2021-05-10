use crate::TestCaseResults;

const RED: &'static str = "rgb(255, 0, 0)";
const GREEN: &'static str = "rgb(50, 205, 50)";
const BLACK: &'static str = "rgb(0, 0, 0)";

/// Constructs an HTML visualization of a test suite.
pub struct ViewHtml {}

impl ViewHtml {
    #[allow(missing_docs)]
    pub fn new() -> Self {
        ViewHtml {}
    }

    /// Constructs an HTML visualization of a test suite.
    pub fn process_test_results(&self, test_results: &TestCaseResults) -> String {
        let mut output = format!(
            r#"<html>
  <body>
    <h1>{title}</h1>
    <p>{description}</p>"#,
            title = test_results.suite_title(),
            description = test_results.suite_description(),
        );

        for test_result in test_results.results() {
            let ok_or_failed = if test_result.did_pass() {
                "(ok)"
            } else {
                "(FAILED)"
            };

            output += &format!(
                r#"

    <div style="margin-bottom: 20px;">
      <div style="font-size: 24px; font-weight: bold;">
        {title}
        <label style="color: {ok_or_passed_color};"> {ok_or_failed}</label>
        <p style="color: {description_color}; font-size: 14px; margin: 0px;">{description}</p>
      </div>
      {html_visual}
    </div>"#,
                title = test_result.title(),
                description = test_result.description(),
                ok_or_passed_color = if test_result.did_pass() { GREEN } else { RED },
                description_color = if test_result.did_pass() { BLACK } else { RED },
                ok_or_failed = ok_or_failed,
                html_visual = test_result.metadata().get("html-visual").unwrap()
            );
        }

        output += &format!(
            r#"
  </body>
</html>"#,
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
        let results = ViewHtml::new().process_test_results(&test_results);

        let expected = r#"<html>
  <body>
    <h1>My Test Suite Title</h1>
    <p>My Test Suite description.</p>

    <div style="margin-bottom: 20px;">
      <div style="font-size: 24px; font-weight: bold;">
        Test Case Title
        <label style="color: rgb(50, 205, 50);"> (ok)</label>
        <p style="color: rgb(0, 0, 0); font-size: 14px; margin: 0px;">Test Case Description</p>
      </div>
      <div><em>Test case html visualization here</em></div>
    </div>
  </body>
</html>"#;

        assert_eq!(results, expected)
    }

    /// Verify the output of one failing test case.
    #[test]
    fn one_test_case_failed() {
        let test_results = test_suite_one(PassedOrFailed::Failed);
        let results = ViewHtml::new().process_test_results(&test_results);

        let expected = r#"<html>
  <body>
    <h1>My Test Suite Title</h1>
    <p>My Test Suite description.</p>

    <div style="margin-bottom: 20px;">
      <div style="font-size: 24px; font-weight: bold;">
        Test Case Title
        <label style="color: rgb(255, 0, 0);"> (FAILED)</label>
        <p style="color: rgb(255, 0, 0); font-size: 14px; margin: 0px;">Test Case Description</p>
      </div>
      <div><em>Test case html visualization here</em></div>
    </div>
  </body>
</html>"#;

        assert_eq!(results, expected)
    }

    /// Verify the output of one passing and one failing test case.
    #[test]
    fn two_test_cases_one_pass_one_fail() {
        let test_results = test_suite_two(PassedOrFailed::Passed, PassedOrFailed::Failed);
        let results = ViewHtml::new().process_test_results(&test_results);

        let expected = r#"<html>
  <body>
    <h1>My Test Suite Title</h1>
    <p>My Test Suite description.</p>

    <div style="margin-bottom: 20px;">
      <div style="font-size: 24px; font-weight: bold;">
        Test Case Title
        <label style="color: rgb(50, 205, 50);"> (ok)</label>
        <p style="color: rgb(0, 0, 0); font-size: 14px; margin: 0px;">Test Case Description</p>
      </div>
      <div><em>Test case html visualization here</em></div>
    </div>

    <div style="margin-bottom: 20px;">
      <div style="font-size: 24px; font-weight: bold;">
        Test Case Title
        <label style="color: rgb(255, 0, 0);"> (FAILED)</label>
        <p style="color: rgb(255, 0, 0); font-size: 14px; margin: 0px;">Test Case Description</p>
      </div>
      <div><em>Test case html visualization here</em></div>
    </div>
  </body>
</html>"#;

        assert_eq!(results, expected)
    }

    fn test_suite_one(pass_fail: PassedOrFailed) -> TestCaseResults {
        let mut result = TestCaseResult::new(
            test_case_title(),
            test_case_description(),
            pass_fail.did_pass(),
        );
        result.insert_metadata("html-visual".to_string(), html_visual());

        TestCaseResults::new(test_suite_title(), test_suite_description(), vec![result])
    }

    fn test_suite_two(pass_fail_1: PassedOrFailed, pass_fail_2: PassedOrFailed) -> TestCaseResults {
        let mut result1 = TestCaseResult::new(
            test_case_title(),
            test_case_description(),
            pass_fail_1.did_pass(),
        );
        result1.insert_metadata("html-visual".to_string(), html_visual());
        let mut result2 = TestCaseResult::new(
            test_case_title(),
            test_case_description(),
            pass_fail_2.did_pass(),
        );
        result2.insert_metadata("html-visual".to_string(), html_visual());

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

    fn html_visual() -> String {
        "<div><em>Test case html visualization here</em></div>".to_string()
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
