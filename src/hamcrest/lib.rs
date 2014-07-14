#![feature(macro_rules)]

extern crate collections;
extern crate debug;

pub use matchers::is::{is,is_not};
pub use not = matchers::is::is_not;
pub use matchers::none::{none};
pub use matchers::equal_to::equal_to;
pub use matchers::existing_path::{existing_path,existing_file,existing_dir};
pub use matchers::vecs::{of_len,contains};

pub use core::{
    BaseMatcher,
    Matcher,
    Match,
    assert_that,
    mismatch,
    expect
};

#[macro_escape]
pub mod core;
pub mod matchers {
  pub mod equal_to;
  pub mod existing_path;
  pub mod is;
  pub mod none;
  pub mod vecs;
}
