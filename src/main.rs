// A simple program to exercise the functionality.
// Copyright (C) 2024  Sohum Mendon
// SPDX-License-Identifier: MIT

#![deny(clippy::pedantic)]
use std::{env, path::PathBuf};

use nestedfolder::resolve;

fn main() {
    let mut args = env::args_os().skip(1).peekable();

    if args.peek().is_none() {
        eprintln!("usage: nestedfolder [path..]");
        return;
    }

    for arg in args.filter_map(|p| {
        if p.is_empty() {
            None
        } else {
            Some(PathBuf::from(p))
        }
    }) {
        match resolve(&arg) {
            Ok(p) => println!("{}", p.display()),
            Err(why) => eprintln!("failed for {arg:?}: {why}"),
        }
    }
}
