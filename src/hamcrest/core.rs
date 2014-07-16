use std::fmt::Show;

pub fn assert_that<T>(actual: &T, matcher: Box<Matcher<T>>) {
    assert_with(actual, matcher, || {}, |msg| fail!("\n{}", msg));
}

// Useful for testing matchers
fn assert_with<T>(actual: &T, matcher: Box<Matcher<T>>, succ: ||, err: |String|) {
    match matcher.matches(actual) {
        Success => succ(),
        Failure(message) => err(message)
    }
}

pub fn expect_fail<T, S: Show>(actual: &T, matcher: Box<Matcher<T>>, msg: S) {
    use equal_to;
    assert_with(actual, matcher,
        || fail!("Matcher should not succeed"),
        |err| assert_that(&err, equal_to(msg.to_string())));
}

pub fn mismatch<T: Show, U: Show>(actual: T, expected: U) -> String {
    format!("Expected: {}\n     Got: {}", expected, actual)
}

pub enum Match {
    Success,
    Failure(String)
}

pub trait Matcher<T> {
    fn matches(&self, actual: &T) -> Match;
    fn describe(&self) -> String;

    fn failure_message_when_negated(&self) -> String {
        format!("Expected: not {}", self.describe())
    }
}

pub fn expect(expect: bool, failure: String) -> Match {
    if expect {
        Success
    } else {
        Failure(failure)
    }
}
