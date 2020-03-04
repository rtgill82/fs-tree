# fs-tree

_An iterator that traverses an entire directory tree._

**NOTICE:** I'm deprecating this crate in favor of [`walkdir`][walkdir],
which I did not notice before I wrote this. It's better named and
probably better written.

[walkdir]: https://crates.io/crates/walkdir

## Description

An iterater that traverses and entire directory tree. It can be
configured to exclude specific files or entire directory trees.
A minimum or maximum directory depth may also be set.

## Example Usage

```rust
extern crate fs_tree;
use fs_tree::FsTreeBuilder;

fn main() {
    let fs_tree = FsTreeBuilder::new("/var")
        .max_depth(3)
        .ignore_files(&["/var/log/lastlog"])
        .build();

    for file in fs_tree {
        println!("{:?}", file);
    }
}
```

## License

Copyright (C) 2020 Robert Gill <<locke@sdf.org>>

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to
deal in the Software without restriction, including without limitation the
rights to use, copy, modify, merge, publish, distribute, sublicense, and/or
sell copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in
all copies of the Software, its documentation and marketing & publicity
materials, and acknowledgment shall be given in the documentation, materials
and software packages that this Software was used.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL
THE AUTHORS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER
IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
