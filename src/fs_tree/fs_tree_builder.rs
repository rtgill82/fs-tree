use std::cell::RefCell;
use std::convert::AsRef;
use std::path::{Path,PathBuf};

use crate::fs_tree::FsTree;

///
/// Configure and build an `FsTree` iterator.
///
#[derive(Default)]
pub struct FsTreeBuilder
{
    path: PathBuf,
    ignore_files: Option<Vec<PathBuf>>,
    ignore_paths: Option<Vec<PathBuf>>,
    max_depth: Option<usize>,
    min_depth: usize
}

impl FsTreeBuilder
{
    ///
    /// Create a new `FsTreeBuilder` with a root directory of `path`.
    ///
    pub fn new<P>(path: P) -> FsTreeBuilder
        where P: AsRef<Path>
    {
        let path = path.as_ref().to_path_buf();
        FsTreeBuilder { path, ..Default::default() }
    }

    ///
    /// Ignore specified files in the iterator (chainable).
    ///
    pub fn ignore_files<P>(mut self, paths: &[P]) -> Self
        where P: AsRef<Path>
    {
        Self::_set_ignore_files(&mut self, paths);
        self
    }

    ///
    /// Ignore specified files in the iterator.
    ///
    pub fn set_ignore_files<P>(&mut self, paths: &[P])
        where P: AsRef<Path>
    {
        Self::_set_ignore_files(self, paths);
    }

    ///
    /// Ignore specified directory trees in the iterator (chainable).
    ///
    pub fn ignore_paths<P>(mut self, paths: &[P]) -> Self
        where P: AsRef<Path>
    {
        Self::_set_ignore_paths(&mut self, paths);
        self
    }

    ///
    /// Ignore specified directory trees in the iterator.
    ///
    pub fn set_ignore_paths<P>(&mut self, paths: &[P])
        where P: AsRef<Path>
    {
        Self::_set_ignore_paths(self, paths);
    }

    ///
    /// Set a maximum directory depth (chainable).
    ///
    pub fn max_depth(mut self, value: usize) -> Self {
        self.max_depth = Some(value);
        self
    }

    ///
    /// Set a maximum directory depth.
    ///
    pub fn set_max_depth(&mut self, value: usize) {
        self.max_depth = Some(value);
    }

    ///
    /// Set a minimum directory depth (chainable).
    ///
    pub fn min_depth(mut self, value: usize) -> Self {
        self.min_depth = value;
        self
    }

    ///
    /// Set a minimimum directory depth.
    ///
    pub fn set_min_depth(&mut self, value: usize) {
        self.min_depth = value;
    }

    ///
    /// Create the `FsTree` iterator.
    ///
    pub fn build(self) -> FsTree {
        FsTree {
            top: self.path,
            stack: RefCell::new(Vec::new()),
            ignore_files: self.ignore_files,
            ignore_paths: self.ignore_paths,
            max_depth: self.max_depth,
            min_depth: self.min_depth,
            ..Default::default()
        }
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
