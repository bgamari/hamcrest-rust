use core::{Matcher,SelfDescribing,Description};

#[deriving(Clone,Eq)]
pub enum PathType {
  AnyType,
  File,
  Dir
}

#[deriving(Clone,Eq)]
pub struct ExistingPath {
  path_type: PathType
}

impl SelfDescribing for ExistingPath {
  fn describe_to(&self, desc: &mut Description) {
    desc.append_text("an existing file");
  }
}

impl Matcher<Path> for ExistingPath {
  fn matches(&self, actual: &Path) -> bool {
    if !actual.exists() {
      return false;
    }

    match self.path_type {
      File => actual.is_file(),
      Dir => actual.is_dir(),
      AnyType => true
    }
  }

  fn describe_mismatch(&self, actual: &Path, desc: &mut Description) {
    desc.append_text(format!("`{}` was missing", actual.display()));
  }
}

pub fn existing_path() -> ExistingPath {
  ExistingPath { path_type: AnyType }
}

pub fn existing_file() -> ExistingPath {
  ExistingPath { path_type: File }
}

pub fn existing_dir() -> ExistingPath {
  ExistingPath { path_type: Dir }
}

#[cfg(test)]
mod test {
  use std::os;

  #[test]
  fn test_stuff() {
    println!("env: {}", os::getenv("FOO"));
    fail!("not implemented");
  }
}
