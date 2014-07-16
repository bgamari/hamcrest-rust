use {Matcher, Match, expect};

#[deriving(Show)]
pub enum PathType {
    AnyType,
    File,
    Dir
}

impl PathType {
    fn was_not(&self, actual: &Path) -> String {
        format!("{} is not {}", actual.display(), self.describe())
    }

    fn describe(&self) -> String {
        match *self {
            File => format!("a file"),
            Dir => format!("a directory"),
            AnyType => format!("any kind of file")
        }
    }
}

pub struct ExistingPath {
    expected: PathType
}

impl ExistingPath {
    fn match_path_type(&self, actual: &Path) -> Match {
        let message = self.expected.was_not(actual);

        match self.expected {
            File => expect(actual.is_file(), message),
            Dir => expect(actual.is_dir(), message),
            AnyType => expect(actual.exists(), message)
        }
    }
}

impl Matcher<Path> for ExistingPath {
    fn matches(&self, actual: &Path) -> Match {
        self.match_path_type(actual)
    }

    fn describe(&self) -> String {
        format!("file was {}", self.expected.describe())
    }
}

pub fn existing_path() -> Box<ExistingPath> {
    box ExistingPath { expected: AnyType }
}

pub fn existing_file() -> Box<ExistingPath> {
    box ExistingPath { expected: File }
}

pub fn existing_dir() -> Box<ExistingPath> {
    box ExistingPath { expected: Dir }
}

#[cfg(test)]
mod test {
    use std::os;
    use {assert_that,is,is_not,existing_file,existing_dir,existing_path};

    #[test]
    fn test_with_existing_file() {
        let path = path(os::getenv("TEST_EXISTS_FILE"), "./README.md");

        assert_that(&path, is(existing_path()));
        assert_that(&path, is(existing_file()));
        assert_that(&path, is_not(existing_dir()));
    }

    #[test]
    fn test_with_existing_dir() {
        let path = path(os::getenv("TEST_EXISTS_DIR"), "./target");

        assert_that(&path, is(existing_path()));
        assert_that(&path, is(existing_dir()));
        assert_that(&path, is_not(existing_file()));
    }

    #[test]
    fn test_with_nonexisting_path() {
        let path = path(os::getenv("TEST_EXISTS_NONE"), "./zomg.txt");

        assert_that(&path, is_not(existing_path()));
        assert_that(&path, is_not(existing_file()));
        assert_that(&path, is_not(existing_dir()));
    }

    fn path(path: Option<String>, default: &str) -> Path {
        Path::new(path.unwrap_or(default.to_string()))
    }
}
