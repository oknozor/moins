extern crate moins;
extern crate termion;

use io::Result;
use std::env;
use std::fs::File;
use std::io;
use std::io::prelude::*;

use moins::Pager;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    let path = if args.len() > 1 {
        Some(args[1].as_str())
    } else {
        None
    };

    let mut content = String::new();

    let mut file = if let Some(path) = path {
        File::open(path)?
    } else {
        panic!("expected an input")
    };

    file.read_to_string(&mut content)?;

    Pager::run(&mut content, None);

    Ok(())
}
