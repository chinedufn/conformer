use conformer::{run_test_suite, TestCase, TestCaseResult, TestSuite, ViewHtml, ViewSimpleText};
use renderer_test_suite::{MySimpleRendererImplementation, RenderCommand, SimpleRenderer};
use std::any::Any;
use std::path::PathBuf;

const RED: [u8; 4] = [255, 0, 0, 255];
const BLUE: [u8; 4] = [0, 0, 255, 255];

fn main() {
    let test_cases: Vec<Box<dyn SimpleRendererTestCase>> = vec![
        Box::new(EntirePixelBufferTestCase {
            title: "All Red pixels",
            width: 256,
            height: 256,
            command: RenderCommand::AllRed,
            expected_pixels: RED.repeat(256 * 256),
        }),
        Box::new(EntirePixelBufferTestCase {
            title: "All Blue Pixels",
            width: 300,
            height: 300,
            command: RenderCommand::AllBlue,
            expected_pixels: BLUE.repeat(300 * 300),
        }),
        Box::new(FirstPixelTestCase {
            title: "Uses final color command.",
            description: r#"Verify that the final color is that of the final command."#,
            width: 200,
            height: 200,
            commands: vec![RenderCommand::AllRed, RenderCommand::AllBlue],
            expected_pixel: BLUE,
        }),
    ];
    let test_cases: Vec<Box<dyn TestCase<Box<dyn SimpleRenderer>>>> = test_cases
        .into_iter()
        .map(|t| Box::new(t) as Box<dyn TestCase<Box<dyn SimpleRenderer>>>)
        .collect();

    let test_results = run_test_suite(
        |test_case| {
            let test_case = test_case.as_any();
            let test_case = test_case
                .downcast_ref::<Box<dyn SimpleRendererTestCase>>()
                .unwrap();

            Box::new(MySimpleRendererImplementation::new(
                test_case.width(),
                test_case.height(),
            ))
        },
        TestSuite::new(
            "Simple Renderer Test Suite".to_string(),
            "Various tests to ensure that an implementation of SimpleRenderer is working properly."
                .to_string(),
            test_cases,
        ),
    );

    let simple_visual = ViewSimpleText::new().process_test_results(&test_results);
    println!("{}", simple_visual);

    let html_visual = ViewHtml::new().process_test_results(&test_results);
    let out_dir = target_dir().join("html-visual");
    let out_html_file = out_dir.join("index.html");

    std::fs::create_dir_all(out_dir).unwrap();
    std::fs::write(&out_html_file, html_visual).unwrap();

    println!(
        r#"HTML test suite visual written to:
{}

You can visualize with any http server such as:

cd target/html-visual
python -m SimpleHTTPServer 8080
open http://localhost:8000
"#,
        out_html_file.to_str().unwrap()
    );

    test_results.assert_did_pass();
}

/// Test suites might be composed of many different kinds of test cases.
///
/// This example test case is one that checks the entire pixel buffer of your renderer and
/// verifies that it contains the correct colors.
struct EntirePixelBufferTestCase {
    title: &'static str,
    command: RenderCommand,
    expected_pixels: Vec<u8>,
    width: u32,
    height: u32,
}

/// Test suites might be composed of many different kinds of test cases.
///
/// This example test case is one that checks the first pixel of your renderer and
/// verifies that it contains the correct colors.
///
/// Note that this isn't a particularly realistic test, we're simply illustrating that it's
/// possible to have multiple different kinds of test cases within your test suite.
struct FirstPixelTestCase {
    title: &'static str,
    description: &'static str,
    commands: Vec<RenderCommand>,
    expected_pixel: [u8; 4],
    width: u32,
    height: u32,
}

/// Sometimes your type that you are using the test suite on will need to be initialized in
/// different ways depending on the test case.
///
/// To handle this you may create a trait that all of your test case types implement. Then, when
/// initializing instances of the type that you are using the test suite on you can use the methods
/// from this trait.
trait SimpleRendererTestCase: TestCase<Box<dyn SimpleRenderer>> {
    /// The width of the pixel buffer to test.
    fn width(&self) -> u32;

    /// The height of the pixel buffer to test.
    fn height(&self) -> u32;
}

impl TestCase<Box<dyn SimpleRenderer>> for EntirePixelBufferTestCase {
    fn run(self: Box<Self>, mut simple_renderer: Box<dyn SimpleRenderer>) -> TestCaseResult {
        simple_renderer.render(&vec![self.command]);

        let actual_rgba_pixels = simple_renderer.rgba_pixels();

        let did_pass = &actual_rgba_pixels == &self.expected_pixels;

        let html_visual = self.make_test_results_html(&actual_rgba_pixels);

        let mut test_case_result =
            TestCaseResult::new(self.title.to_string(), "".to_string(), did_pass);
        test_case_result.insert_metadata("html-visual".to_string(), html_visual);

        test_case_result
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl TestCase<Box<dyn SimpleRenderer>> for FirstPixelTestCase {
    fn run(self: Box<Self>, mut simple_renderer: Box<dyn SimpleRenderer>) -> TestCaseResult {
        simple_renderer.render(&self.commands);

        let rgba_pixels = simple_renderer.rgba_pixels();
        let actual = &rgba_pixels[0..4];

        let did_pass = actual == &self.expected_pixel;

        let html_visual = if did_pass {
            let pixel_html = self.make_pixel_html(self.expected_pixel);

            format!(
                r#"
<h4>Success</h4> 
{pixel_html}
</div>
"#,
                pixel_html = pixel_html
            )
        } else {
            let a = actual;
            let actual_pixel_html = self.make_pixel_html([a[0], a[1], a[2], a[3]]);
            let expected_pixel_html = self.make_pixel_html(self.expected_pixel);

            format!(
                r#"<div>
  <div>
    The first pixel was {actual:?}
    {actual_pixel_html}
  </div>  
  <div>
    It should have been {expected:?}
    {expected_pixel_html}
  </div>
</div> 
        "#,
                actual = actual,
                expected = &self.expected_pixel,
                actual_pixel_html = actual_pixel_html,
                expected_pixel_html = expected_pixel_html
            )
        };

        let mut test_case_result = TestCaseResult::new(
            self.title.to_string(),
            self.description.to_string(),
            did_pass,
        );
        test_case_result.insert_metadata("html-visual".to_string(), html_visual);

        test_case_result
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl TestCase<Box<dyn SimpleRenderer>> for Box<dyn SimpleRendererTestCase> {
    fn run(self: Box<Self>, type_to_test: Box<dyn SimpleRenderer>) -> TestCaseResult {
        (*self).run(type_to_test)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl SimpleRendererTestCase for EntirePixelBufferTestCase {
    fn width(&self) -> u32 {
        self.width
    }

    fn height(&self) -> u32 {
        self.height
    }
}

impl SimpleRendererTestCase for FirstPixelTestCase {
    fn width(&self) -> u32 {
        self.width
    }

    fn height(&self) -> u32 {
        self.height
    }
}

impl FirstPixelTestCase {
    fn make_pixel_html(&self, color: [u8; 4]) -> String {
        format!(
            r#"
        <div style="width: {width}px; height: {height}px; background-color: rgba({r}, {g}, {b}, {a});">
        </div>
        "#,
            width = self.width,
            height = self.height,
            r = color[0],
            g = color[1],
            b = color[2],
            a = color[3],
        )
    }
}

impl EntirePixelBufferTestCase {
    fn make_test_results_html(&self, rgba_pixels: &[u8]) -> String {
        let width = self.width;
        let height = self.height;

        let total_rgba_bytes = width * height * 4;

        let mut actual_png = Vec::with_capacity(total_rgba_bytes as usize);
        let mut expected_png = Vec::with_capacity(total_rgba_bytes as usize);

        {
            let mut actual_encoder = png::Encoder::new(&mut actual_png, width, height);
            actual_encoder.set_color(png::ColorType::RGBA);
            actual_encoder.set_depth(png::BitDepth::Eight);

            let mut expected_encoder = png::Encoder::new(&mut expected_png, width, height);
            expected_encoder.set_color(png::ColorType::RGBA);
            expected_encoder.set_depth(png::BitDepth::Eight);

            let mut writer = actual_encoder.write_header().unwrap();
            writer.write_image_data(rgba_pixels).unwrap();

            let mut writer = expected_encoder.write_header().unwrap();
            writer.write_image_data(&self.expected_pixels).unwrap();
        }

        let actual = base64::encode(actual_png);
        let expected = base64::encode(expected_png);

        format!(
            r#"
<div style="margin-bottom: 20px;">
  <div style="display: flex; flex-wrap: wrap;">
    <div style="margin-right: 10px">
      <div>Actual</div>
      <img src="data:image/png;base64,{actual}" />
    </div>
    <div>
      <div>Expected</div>
      <img src="data:image/png;base64,{expected}" />
    </div>
  </div>
</div>
            "#,
            actual = actual,
            expected = expected
        )
    }
}

fn target_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../target")
}
