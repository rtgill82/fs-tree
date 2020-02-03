use std::path::{Path,PathBuf};
use std::{error,fmt,io};

#[derive(Debug)]
pub struct Error {
    entry: PathBuf,
    source: io::Error
}

impl Error {
    pub fn new<P>(entry: P, source: io::Error) -> Error
        where P: AsRef<Path>
    {
        let entry = entry.as_ref().to_path_buf();
        Error { entry, source }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.entry.to_string_lossy(), self.source)
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        Some(&self.source)
    }
}
