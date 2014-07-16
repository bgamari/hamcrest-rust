use std::fmt::Show;
use {Matcher, Match, expect};

pub struct IsNone<T>;

impl<T: Show> Matcher<Option<T>> for IsNone<T> {
    fn describe(&self) -> String {
        "None".to_string()
    }

    fn matches(&self, actual: &Option<T>) -> Match {
        expect(actual.is_none(), format!("was {}", actual))
    }
}

pub fn none<T>() -> Box<IsNone<T>> {
    box IsNone
}

#[cfg(test)]
mod test {
    use {assert_that,is,is_not,none};
    use core::expect_fail;

    #[test]
    fn test_none_is_none() {
        assert_that(&None, is(none::<int>()));
        assert_that(&Some(1), is_not(none::<int>()));

        expect_fail(&None, is_not(none::<int>()), "Expected: not None");
    }
}
