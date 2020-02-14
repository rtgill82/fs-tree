use std::cell::RefMut;
use std::path::PathBuf;
use std::result;

use crate::Error;
use crate::fs_tree::FsTree;
use crate::fs_tree_iter::FsTreeIter;
use crate::fs_tree::Result;
use crate::read_dir::ReadDir;

pub struct Iter<'a>(pub(crate) &'a FsTree);

impl<'a> FsTreeIter for Iter<'a> {
    fn top(&self) -> &PathBuf {
        self.0.top()
    }

    fn stack(&self) -> RefMut<Vec<ReadDir>> {
        self.0.stack()
    }

    fn push_dir(&self, path: &PathBuf) -> result::Result<(), Error> {
        self.0.push_dir(path)
    }

    fn ignore_file(&self, path: &PathBuf) -> bool {
        self.0.ignore_file(path)
    }

    fn ignore_path(&self, path: &PathBuf) -> bool {
        self.0.ignore_path(path)
    }

    fn depth(&self) -> usize {
        self.0.depth()
    }

    fn max_depth(&self) -> Option<usize> {
        self.0.max_depth()
    }

    fn min_depth(&self) -> usize {
        self.0.min_depth()
    }
}

impl<'a> Iterator for Iter<'a> {
    type Item = Result;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_iter()
    }
}
