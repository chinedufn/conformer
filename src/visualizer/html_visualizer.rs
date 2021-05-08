use crate::{MetadataValue, TestCaseResults};

// TODO: Move into a conformer-html-visualizer crate if we end up needing dependencies.
struct HtmlVisualizer {}

impl HtmlVisualizer {
    pub fn process_test_results(&self, results: &TestCaseResults) -> String {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn todo() {
        unimplemented!(
            r#"
Test the HTML output here.
        "#
        )
    }
}
