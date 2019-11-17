extern crate moins;

use moins::Color;
use moins::Pager;
use moins::PagerOptions;
use std::collections::HashMap;

fn main() {
    let mut content = "A noir, E blanc, I rouge, U vert, O bleu, voyelles,
Je dirai quelque jour vos naissances latentes.
A, noir corset velu des mouches éclatantes
Qui bombillent autour des puanteurs cruelles"
        .to_owned();

    let mut colors = HashMap::new();
    colors.insert("A".into(), Color::Black);
    colors.insert("E".into(), Color::White);
    colors.insert("I".into(), Color::Red);
    colors.insert("U".into(), Color::Green);
    colors.insert("O".into(), Color::Blue);

    let options = PagerOptions {
        colors,
        search: false,
        line_number: false,
    };

    Pager::run(&mut content, Some(options));
}
