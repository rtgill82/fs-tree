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
    fn top(&self) -> &PathBuf;
    fn stack(&self) -> RefMut<Vec<ReadDir>>;
    fn push_dir(&self, path: &PathBuf) -> StdResult<(), Error>;
    fn pop_dir(&self);
    fn ignore_file(&self, path: &PathBuf) -> bool;
    fn ignore_path(&self, path: &PathBuf) -> bool;
    fn depth(&self) -> usize;
    fn max_depth(&self) -> Option<usize>;
    fn min_depth(&self) -> usize;

    fn next_entry(&self) -> Option<Result> {
        loop {
            let mut stack = self.stack();
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
                            drop(stack);
                            continue;
                        } else {
                            return Some(Ok(entry));
                        }
                    }
                },

                None => {
                    drop(stack);
                    self.pop_dir();
                    continue;
                }
            };
        }
    }

    fn next_iter(&self) -> Option<Result> {
        if self.depth() == 0 {
            try_opt!(self.push_dir(self.top()));
            let path = self.top().clone();
            if !self.ignore_file(&path) && self.min_depth() == 0 {
                return Some(Ok(path));
            }
        }

        if self.max_depth() == Some(0) { return None; }
        loop {
            let entry =  try_opt!(self.next_entry()?);
            let stat = fs::symlink_metadata(&entry);
            if stat.is_err() {
                return Some(Err(Error::new(&entry, stat.unwrap_err())));
            }

            let depth = self.depth();
            let stat = stat.unwrap();
            let ignore = self.ignore_path(&entry);
            if !ignore && !stat.file_type().is_symlink() && stat.is_dir() {
                if let Err(err) = self.push_dir(&entry) {
                    return Some(Err(err));
                }
            }

            if depth < self.min_depth() {
                continue;
            }

            return Some(Ok(entry));
        }
    }
}
