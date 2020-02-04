use std::convert::AsRef;
use std::path::{Path,PathBuf};

use crate::error::Error;
use crate::fs_tree::FsTree;
use crate::fs_tree::read_dir::ReadDir;

#[derive(Default)]
pub struct FsTreeBuilder
{
    path: PathBuf,
    ignore_paths: Option<Vec<PathBuf>>,
    max_depth: Option<usize>
}

impl FsTreeBuilder
{
    pub fn new<P>(path: P) -> FsTreeBuilder
        where P: AsRef<Path>
    {
        let path = path.as_ref().to_path_buf();
        FsTreeBuilder { path, ..Default::default() }
    }

    pub fn ignore_paths<P>(mut self, paths: &[P]) -> Self
        where P: AsRef<Path>
    {
        Self::_set_ignore_paths(&mut self, paths);
        self
    }

    pub fn set_ignore_paths<P>(&mut self, paths: &[P])
        where P: AsRef<Path>
    {
        Self::_set_ignore_paths(self, paths);
    }

    pub fn max_depth(mut self, value: usize) -> Self {
        self.max_depth = Some(value);
        self
    }

    pub fn set_max_depth(&mut self, value: usize) {
        self.max_depth = Some(value);
    }

    pub fn build(self) -> Result<FsTree, Error> {
        let mut stack = Vec::new();
        let path = ReadDir::new(self.path)?;
        stack.push(path);

        Ok(FsTree {
            stack, 
            ignore_paths: self.ignore_paths,
            max_depth: self.max_depth
        })
    }

    fn _set_ignore_paths<P>(&mut self, paths: &[P])
        where P: AsRef<Path>
    {
        let ignore_paths = paths.iter()
            .map(|p| {
                p.as_ref().to_path_buf()
            }).collect();
        self.ignore_paths = Some(ignore_paths);
    }
}
