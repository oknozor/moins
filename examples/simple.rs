extern crate moins;

use moins::Moins;

fn main() {
    let mut content = String::from("👋 🌎!");
    Moins::run(&mut content, None);
}
