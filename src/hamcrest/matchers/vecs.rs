use std::fmt::Show;
use std::vec::Vec;
use core::{Success, Failure, BaseMatcher, expect};
use {
    Matcher,
    Match,
    mismatch,
};

#[deriving(Clone)]
pub struct OfLen {
    len: uint
}

impl<T> BaseMatcher<Vec<T>> for OfLen {
    fn description(&self) -> &'static str {
        "of len"
    }
}

impl<T> Matcher<Vec<T>> for OfLen {
    fn matches(&self, actual: &Vec<T>) -> Match {
        let failure: String = mismatch(
            format!("it had {} elements", actual.len()),
            format!("a vec of length {}", self.len));

        expect(self.len == actual.len(), failure)
    }
}

pub fn of_len(len: uint) -> Box<OfLen> {
    box OfLen { len: len }
}

#[deriving(Clone)]
pub struct Contains<T> {
    items: Vec<T>,
    exactly: bool
}

impl<T> Contains<T> {
    pub fn exactly(mut ~self) -> Box<Contains<T>> {
        self.exactly = true;
        self
    }
}

impl<T: Show + PartialEq + Clone> BaseMatcher<Vec<T>> for Contains<T> {
    fn description(&self) -> &'static str {
        "contains"
    }
}

impl<T : Show + PartialEq + Clone> Matcher<Vec<T>> for Contains<T> {
    fn matches(&self, actual: &Vec<T>) -> Match {
        let mut rem = actual.clone();

        for item in self.items.iter() {
            match rem.iter().position(|a| *item == *a) {
                Some(idx) => { rem.remove(idx); },
                None => return Failure(format!("was {}", actual))
            }
        }

        if self.exactly && !rem.is_empty() {
            return Failure(format!("also had {}", rem));
        }

        Success
    }
}

pub fn contains<T>(items: Vec<T>) -> Box<Contains<T>> {
    box Contains { items: items, exactly: false }
}

#[cfg(test)]
mod test {
    use {
        not,
        assert_that,
        contains
    };

    #[test]
    fn test_vec_contains() {
        assert_that(&vec!(1i, 2, 3), contains(vec!(1i, 2)));
        assert_that(&vec!(1i, 2, 3), not(contains(vec!(4i))));
    }

    #[test]
    fn test_vec_contains_exactly() {
        assert_that(&vec!(1i, 2, 3), contains(vec!(1i, 2, 3)).exactly());
        assert_that(&vec!(1i, 2, 3), not(contains(vec!(1i, 2)).exactly()));
    }
}
