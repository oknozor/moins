# Moins [![Latest Version]][crates.io] [![Build Status]][travis]

[Build Status]: https://travis-ci.org/oknozor/moins.svg?branch=master
[travis]: https://travis-ci.org/oknozor/musicbrainz_rs
[Latest Version]: https://img.shields.io/crates/v/moins.svg
[crates.io]: https://www.crates.io/crates/moins


🥖 *"moins"* is french for *"less"* 🥖 ! .

## What is moins ?

Moins is a Keep It Simple Stupid less like pager that you can use as a crate.

It aims to be used in other cli app project, you probably don't want to use it as a replacement for less, cause it does less than less.

## How to

Add moins to your `Cargo.toml` dependencies.

```toml
[dependencies]
moins = "0.1.0"
```

Moins expose a single `run` function and a `PagerOption` struct to define your pager behavior.

Here is the simplest pager you can run :

```rust
extern crate moins;

use moins::Moins;

fn main() {
    let mut content = String::from("👋 🌎!");
    Moins::run(&mut content, None);
}
```

If you want to add some colors to the pager you can add pagers options :

```rust
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
```

you can run the examples with the command `cargo run --example my_example`

## Search

WIP

## Line number

WIP

## Key binding

| Key | Action |
|:--  | :---   |
| `Arrow up` | scroll up |
| `k` | scroll up |
| `Arrow down` | scroll down |
| `j` | scroll down |
| `q` | quit |

## Contributing

Contribution are welcome, don't hesitate to submit a PR or fill an issue but keep in mind that moins is a stupid pager. We don't want syntax hilighting, or any fancy feature. If you are looking for something like that [bat](https://github.com/sharkdp/bat) is probably what you need.



