extern crate moins;

use moins::Pager;

fn main() {
    let mut content = String::from("👋 🌎!");
    Pager::run(&mut content, None);
}
