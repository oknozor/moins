extern crate moins;

use moins::Color;
use moins::Moins;
use moins::PagerOptions;
use std::collections::HashMap;

fn main() {
    let mut content = "A noir, E blanc, I rouge, U vert, O bleu, voyelles,
Je dirai quelque jour vos naissances latentes.
A, noir corset velu des mouches éclatantes
Qui bombillent autour des puanteurs cruelles"
        .to_owned();

    let mut colors = HashMap::new();
    colors.insert("A", Color::Black);
    colors.insert("E", Color::White);
    colors.insert("I", Color::Red);
    colors.insert("U", Color::Green);
    colors.insert("O", Color::Blue);

    let options = PagerOptions {
        colors,
        search: false,
        line_number: false,
    };

    Moins::run(&mut content, Some(options));
}
