use std::cell::RefMut;
use std::path::PathBuf;
use std::result::Result as StdResult;
use std::fs;

use crate::error::Error;
use crate::fs_tree::Result;
use crate::read_dir::ReadDir;

macro_rules! try_opt {
    ($e:expr) => {
        match $e {
            Ok(val)  => val,
            Err(err) => return Some(Err(err))
        }
    }
}

pub trait FsTreeIter {
    fn top(&self) -> Option<ReadDir>;
    fn stack(&self) -> RefMut<Vec<ReadDir>>;
    fn ignore_file(&self, path: &PathBuf) -> bool;
    fn ignore_path(&self, path: &PathBuf) -> bool;
    fn depth(&self) -> usize;
    fn max_depth(&self) -> Option<usize>;

    fn push_dir(&self, path: &PathBuf) -> StdResult<(), Error> {
        if let Some(max_depth) = self.max_depth() {
            if self.depth() >= max_depth {
                return Ok(());
            }
        }

        let read_dir = ReadDir::new(path)?;
        self.stack().push(read_dir);
        Ok(())
    }

    fn next_entry(&self) -> Option<Result> {
        let mut stack = self.stack();
        loop {
            let read_dir = match stack.last_mut() {
                Some(read_dir) => read_dir,
                None           => return None
            };

            match read_dir.next() {
                Some(result) => match result {
                    Err(err)  => return Some(Err(err)),

                    Ok(entry) => {
                        let ignore = self.ignore_file(&entry) ||
                                     self.ignore_path(&entry);

                        if ignore {
                            continue;
                        } else {
                            return Some(Ok(entry));
                        }
                    }
                },

                None => {
                    stack.pop();
                    continue;
                }
            };
        }
    }

    fn next_iter(&self) -> Option<Result> {
        if let Some(read_dir) = self.top() {
            let mut stack = self.stack();
            let path = read_dir.path();
            stack.push(read_dir);

            if !self.ignore_file(&path) {
                return Some(Ok(path));
            }
        }

        if self.max_depth() == Some(0) { return None; }
        let entry =  try_opt!(self.next_entry()?);
        let stat = fs::symlink_metadata(&entry);
        if stat.is_err() {
            return Some(Err(Error::new(&entry, stat.unwrap_err())));
        }

        let stat = stat.unwrap();
        let ignore = self.ignore_path(&entry);
        if !ignore && !stat.file_type().is_symlink() && stat.is_dir() {
            if let Err(err) = Self::push_dir(self, &entry) {
                return Some(Err(err));
            }
        }

        Some(Ok(entry))
    }
}
