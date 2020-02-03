use std::path::PathBuf;
use std::result::Result as StdResult;
use std::fs;

pub mod fs_tree_builder;
pub mod read_dir;

use crate::error::Error;
use self::read_dir::ReadDir;

pub type Result = StdResult<PathBuf, Error>;
type OptResult = Option<Result>;

macro_rules! try_opt {
    ($e:expr) => {
        match $e {
            Ok(val)  => val,
            Err(err) => return Some(Err(err)),
        }
    }
}

#[derive(Default)]
pub struct FsTree
{
    stack: Vec<ReadDir>,
    ignore_paths: Option<Vec<PathBuf>>,
    max_depth: Option<usize>
}

impl FsTree
{
    fn next_entry(rd: &mut ReadDir, ign: &Option<Vec<PathBuf>>) -> OptResult {
        while let Some(entry) = try_opt!(rd.next()) {
            if let Some(ignore_paths) = ign {
                if ignore_paths.contains(&entry) {
                    continue;
                }
            }
            return Some(Ok(entry));
        }
        None
    }

    fn push_dir(&mut self, path: &PathBuf) -> StdResult<(), Error> {
        if let Some(max_depth) = self.max_depth {
            if self.stack.len() > max_depth {
                return Ok(());
            }
        }

        let read_dir = ReadDir::new(path)?;
        self.stack.push(read_dir);
        Ok(())
    }
}

impl Iterator for FsTree
{
    type Item = Result;

    fn next(&mut self) -> Option<Self::Item> {
        let ignore_paths = &self.ignore_paths;

        while let Some(read_dir) = self.stack.last_mut() {
            if let Some(result) = Self::next_entry(read_dir, ignore_paths) {
                if result.is_err() {
                    return Some(Err(result.unwrap_err()));
                }

                let entry = result.unwrap();
                let stat = fs::symlink_metadata(&entry);
                if stat.is_err() {
                    return Some(Err(Error::new(&entry, stat.unwrap_err())));
                }

                let stat = stat.unwrap();
                if !stat.file_type().is_symlink() && stat.is_dir() {
                    if let Err(err) = Self::push_dir(self, &entry) {
                        return Some(Err(err));
                    }
                }

                return Some(Ok(entry));
            }
            self.stack.pop();
        }
        None
    }
}
