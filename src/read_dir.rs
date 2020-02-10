use std::convert::AsRef;
use std::path::{Path,PathBuf};
use std::result::Result as StdResult;
use std::{fmt,fs};

use crate::Error;
use crate::fs_tree::Result;

#[derive(Debug)]
pub struct ReadDir
{
    inner: fs::ReadDir,
    path: PathBuf
}

impl ReadDir
{
    pub fn new<P>(path: P) -> StdResult<ReadDir, Error>
        where P: AsRef<Path>
    {
        let inner = fs::read_dir(&path)
            .map_err(|err| Error::new(&path, err))?;
        let path = path.as_ref().to_path_buf();
        Ok(ReadDir { inner, path })
    }

    pub fn next(&mut self) -> Option<Result> {
        self.inner.next().map(|result| {
            match result {
                Ok(entry) => Ok(entry.path()),
                Err(err)  => Err(Error::new(&self.path, err))
            }
        })
    }
}

impl AsRef<Path> for ReadDir {
    fn as_ref(&self) -> &Path {
        self.path.as_ref()
    }
}

impl fmt::Display for ReadDir {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.path.to_string_lossy())
    }
}
