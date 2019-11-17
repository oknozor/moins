extern crate moins;
extern crate termion;

use io::Result;
use std::env;
use std::fs::File;
use std::io;
use std::io::prelude::*;

use moins::Pager;
use moins::PagerOptions;
use moins::Color;
use std::collections::HashMap;

// open your cargo lock with the following command :
// `cargo run --example cli Cargo.lock` and enjoy the colors !
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
        panic!("expected an input file")
    };

    file.read_to_string(&mut content)?;

    let mut colors = HashMap::new();
    colors.insert("[[package]]".into(), Color::Blue);
    colors.insert("dependencies".into(), Color::Magenta);
    colors.insert("version".into(), Color::LightRed);
    colors.insert("name".into(), Color::Cyan);
    colors.insert("metadata".into(), Color::Green);

    let options = PagerOptions {
        colors,
        search: false,
        line_number: false,
    };

    Pager::run(&mut content, Some(options));

    Ok(())
}
