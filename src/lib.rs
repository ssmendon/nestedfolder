// Copyright (C) 2024  Sohum Mendon
// SPDX-License-Identifier: MIT

//! Find the first folder with content in a directory tree.
//!
//! This exports one public method, [`resolve`].

#![deny(clippy::pedantic)]
use std::{
    fs, io,
    path::{Path, PathBuf},
};

/// Iteratively traverse the directory to discover the first non-redundant empty folder.
///
/// There are different cases that we can encounter:
/// 1. The passed in path is not a directory at all.
/// 2. The directory:
///                           contains no files underneath
///                        OR exactly *one file* underneath
///                        OR multiple files and directories.
/// 3. The directory contains exactly one directory underneath.
///
/// # Errors
/// This function may return any error raised by [`fs`]
/// and its directory iteration methods.
pub fn resolve(dir: &Path) -> io::Result<PathBuf> {
    // The implementation is iterative rather than recursive.
    // We hold the current "depth" of the search using a
    // mutable "current" buffer.
    //
    // Then, we have a tracking pointer "candidate" which is
    // `None` when nothing is promising.
    //
    // If we encounter a directory, that becomes our new
    // candidate.
    //
    // If we get to the end at this point,
    // then we have to reset our "current" depth and try again.
    //
    // If any other situation occurs, we immediately return from the function.

    let mut current = PathBuf::from(dir);
    let mut candidate = None;

    // Breaking to 'depth returns from the function immediately.
    'depth: loop {
        let iter = fs::read_dir(&current)?;

        for (index, entry) in iter.enumerate() {
            // Multiple files and directories.
            if index > 0 {
                break 'depth;
            }

            // Exactly one non-directory.
            let entry = entry?;
            if !entry.file_type()?.is_dir() {
                break 'depth;
            }
            candidate = Some(entry.path());
        }

        // If we're here, that means we never changed "candidate".
        // If candidate is changed, it must happen inside the loop.
        // Therefore, if there are no files beneath `dir` we will
        // never end up here.
        if let Some(c) = candidate {
            current = c;
            candidate = None;
        } else {
            break;
        }
    }

    Ok(current)
}
