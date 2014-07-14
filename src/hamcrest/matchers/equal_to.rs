use std::fmt::Show;
use {Matcher, Match, expect};

define_matcher!(EqualTo for T as "equal to" {
    expected: T
})

fn diff<T: Show, U: Show>(expected: &T, actual: &U) -> String {
    format!("Expected: {}\n     Got: {}", expected, actual)
}

impl<T : PartialEq + Show + 'static> Matcher<T> for EqualTo<T> {
    fn matches(&self, actual: &T) -> Match {
        expect(self.expected.eq(actual), diff(&self.expected, actual))
    }

    fn failure_message_when_negated(&self) -> String {
        format!("Expected not: {}", self.expected)
    }
}

pub fn equal_to<T : PartialEq + Show>(expected: T) -> Box<EqualTo<T>> {
    box EqualTo { expected: expected }
}

#[cfg(test)]
mod test {
    use {assert_that,is,equal_to};
    use core::expect_fail;

    #[test]
    fn test_equality_of_ints() {
        // Successful match
        assert_that(&1, is(equal_to(1i)));

        // Unsuccessful match
        expect_fail(&2, equal_to(1i), "Expected: 1\n     Got: 2");
    }
}
