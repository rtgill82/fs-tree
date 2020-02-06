use std::cell::{Cell,RefCell};
use std::convert::AsRef;
use std::path::{Path,PathBuf};

use crate::error::Error;
use crate::fs_tree::FsTree;
use crate::fs_tree::ReadDir;

#[derive(Default)]
pub struct FsTreeBuilder
{
    path: PathBuf,
    ignore_files: Option<Vec<PathBuf>>,
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

    pub fn ignore_files<P>(mut self, paths: &[P]) -> Self
        where P: AsRef<Path>
    {
        Self::_set_ignore_files(&mut self, paths);
        self
    }

    pub fn set_ignore_files<P>(&mut self, paths: &[P])
        where P: AsRef<Path>
    {
        Self::_set_ignore_files(self, paths);
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
        let read_dir = ReadDir::new(self.path)?;
        Ok(FsTree {
            top: Cell::new(Some(read_dir)),
            stack: RefCell::new(Vec::new()),
            ignore_files: self.ignore_files,
            ignore_paths: self.ignore_paths,
            max_depth: self.max_depth
        })
    }

    fn _set_ignore_files<P>(&mut self, paths: &[P])
        where P: AsRef<Path>
    {
        let ignore_files = Self::_path_buf_collect(paths);
        self.ignore_files = Some(ignore_files);
    }

    fn _set_ignore_paths<P>(&mut self, paths: &[P])
        where P: AsRef<Path>
    {
        let ignore_paths = Self::_path_buf_collect(paths);
        self.ignore_paths = Some(ignore_paths);
    }

    fn _path_buf_collect<P>(paths: &[P]) -> Vec<PathBuf>
        where P: AsRef<Path>
    {
        paths.iter().map(|p| {
            p.as_ref().to_path_buf()
        }).collect()
    }
}
