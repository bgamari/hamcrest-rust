use std::fmt::Show;
use std::vec::Vec;
use collections::HashSet;
use {
  success,
  Matcher,
  MatchResult,
  SelfDescribing};

#[deriving(Clone)]
pub struct OfLen {
  len: uint
}

impl SelfDescribing for OfLen {
  fn describe(&self) -> ~str {
    format!("of len {}", self.len)
  }
}

impl<'a, T> Matcher<&'a Vec<T>> for OfLen {
  fn matches(&self, actual: &Vec<T>) -> MatchResult {
    if self.len == actual.len() {
      success()
    }
    else {
      Err(format!("was len {}", actual.len()))
    }
  }
}

pub fn of_len(len: uint) -> ~OfLen {
  ~OfLen { len: len }
}

#[deriving(Clone)]
pub struct Contains<T> {
  items: Vec<T>
}

impl<T : Show> SelfDescribing for Contains<T> {
  fn describe(&self) -> ~str {
    format!("containing {}", self.items)
  }
}

impl<'a, T : Show> Matcher<&'a Vec<T>> for Contains<T> {
  fn matches(&self, actual: &Vec<T>) -> MatchResult {
    let mut rem = HashSet::new();

    for i in range(0, self.items.len()) {
      rem.insert(i);
    }

    for item in actual.iter() {
      return Err(format!("nope"));
    }

    success()
  }
}

pub fn contains<T>(items: Vec<T>) -> ~Contains<T> {
  ~Contains { items: items }
}
