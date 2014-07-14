use std::fmt::Show;
use {Matcher, BaseMatcher, Match, expect};

pub struct IsNone<T>;

impl<T> BaseMatcher<Option<T>> for IsNone<T> {
    fn description(&self) -> &'static str {
        "is none"
    }
}

impl<T: Show> Matcher<Option<T>> for IsNone<T> {
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

    #[test]
    fn test_none_is_none() {
        assert_that(&None, is(none::<int>()));
        assert_that(&Some(1), is_not(none::<int>()));
    }
}
