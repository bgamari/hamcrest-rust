use {Match, Matcher, BaseMatcher};
use core::{Success, Failure};

pub struct Is<M> {
    matcher: Box<M>
}

impl<T, M: Matcher<T>> BaseMatcher<T> for Is<M> {
    fn description(&self) -> &'static str {
        self.matcher.description()
    }
}

impl<T, M: Matcher<T>> Matcher<T> for Is<M> {
    fn matches(&self, actual: &T) -> Match {
        self.matcher.matches(actual)
    }

    fn failure_message_when_negated(&self) -> String {
        self.matcher.failure_message_when_negated()
    }
}

pub fn is<T, M: Matcher<T>>(matcher: Box<M>) -> Box<Is<M>> {
    box Is { matcher: matcher }
}

pub struct IsNot<M> {
    matcher: Box<M>
}

impl<T, M: Matcher<T>> BaseMatcher<T> for IsNot<M> {
    fn description(&self) -> &'static str {
        self.matcher.description()
    }
}

impl<T, M: Matcher<T>> Matcher<T> for IsNot<M> {
    fn matches(&self, actual: &T) -> Match {
        match self.matcher.matches(actual) {
            Success => Failure(self.matcher.failure_message_when_negated()),
            Failure(_) => Success
        }
    }

    fn failure_message_when_negated(&self) -> String {
        fail!("Do not negate an is_not")
    }
}

pub fn is_not<T, M: Matcher<T>>(matcher: Box<M>) -> Box<IsNot<M>> {
    box IsNot { matcher: matcher }
}
