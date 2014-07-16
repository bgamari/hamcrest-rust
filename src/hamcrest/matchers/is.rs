use {Match, Matcher};
use core::{Success, Failure};

pub struct Is<M> {
    matcher: Box<M>
}

impl<T, M: Matcher<T>> Matcher<T> for Is<M> {
    fn matches(&self, actual: &T) -> Match {
        self.matcher.matches(actual)
    }

    fn describe(&self) -> String {
        self.matcher.describe()
    }
}

pub fn is<T, M: Matcher<T>>(matcher: Box<M>) -> Box<Is<M>> {
    box Is { matcher: matcher }
}

pub struct IsNot<M> {
    matcher: Box<M>
}

impl<T, M: Matcher<T>> Matcher<T> for IsNot<M> {
    fn matches(&self, actual: &T) -> Match {
        match self.matcher.matches(actual) {
            Success => Failure(format!("Expected: {}", self.describe())),
            Failure(_) => Success
        }
    }

    fn describe(&self) -> String {
        format!("not {}", self.matcher.describe())
    }

    fn failure_message_when_negated(&self) -> String {
        fail!("Do not negate an is_not")
    }
}

pub fn is_not<T, M: Matcher<T>>(matcher: Box<M>) -> Box<IsNot<M>> {
    box IsNot { matcher: matcher }
}
