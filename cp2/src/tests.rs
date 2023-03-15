pub struct TestResult {
    pub name: &'static str,
    pub pass: bool,
}

pub fn run() -> Vec<TestResult> {
    vec![
        TestResult {
            name: "test 1",
            pass: true,
        },
        TestResult {
            name: "test 2",
            pass: false,
        },
    ]
}
