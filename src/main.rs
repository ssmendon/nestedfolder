use std::{env, path::PathBuf};

use nestedfolder::resolve;

fn main() {
    for arg in env::args_os().skip(1).map(PathBuf::from) {
        println!("{}", resolve(&arg).unwrap().display());
    }
}
