use std::cell::RefCell;
use std::collections::HashMap;
use std::io::Write;
use std::io::{stdin, stdout, Stdout};
use termion::event::Key;
use termion::input::{MouseTerminal, TermRead};
use termion::raw::{IntoRawMode, RawTerminal};
use termion::screen::AlternateScreen;

type Terminal = RefCell<MouseTerminal<AlternateScreen<RawTerminal<Stdout>>>>;

pub struct Moins<'a> {
    lines: Vec<&'a str>,
    height: u16,
    width: u16,
    current_line: usize,
    scroll: usize,
    screen: Terminal,
    options: Option<PagerOptions<'a>>,
}

/// options for `Moins` see the examples
pub struct PagerOptions<'a> {
    /// add color to the matching term
    pub colors: HashMap<&'a str, Color>,
    pub search: bool,
    pub line_number: bool,
}

impl<'a> Moins<'a> {
    /// run moins pager
    pub fn run(content: &'a mut String, options: Option<PagerOptions>) {
        let stdout = stdout().into_raw_mode().unwrap();
        let screen = MouseTerminal::from(AlternateScreen::from(stdout));
        let screen = RefCell::new(screen);

        let mut pager = Moins::new(content, screen, options);

        let stdin = stdin();

        pager.clear();
        pager.write();

        for c in stdin.keys() {
            // Input
            match c.unwrap() {
                Key::Char('q') => {
                    write!(pager.screen.borrow_mut(), "{}", termion::cursor::Show).unwrap();
                    break;
                }
                Key::Down | Key::Char('j') => pager.scroll_down(),
                Key::Up | Key::Char('k') => pager.scroll_up(),
                _ => (),
            }
        }
    }

    fn new(content: &'a mut String, screen: Terminal, options: Option<PagerOptions<'a>>) -> Self {
        let size = termion::terminal_size().unwrap();
        let width = size.0 as usize;
        let mut lines = vec![];

        content.lines().for_each(|line| {
            if line.len() > width {
                lines.push(&line[0..width]);
                lines.push(&line[width..line.len()]);
            } else {
                lines.push(line);
            }
        });

        let height = size.1 as usize;

        let scroll = if lines.len() <= height {
            lines.len()
        } else {
            height
        };

        Moins {
            lines,
            scroll,
            screen,
            current_line: 0,
            height: size.1 - 2,
            width: size.0,
            options,
        }
    }

    fn color(&self, line: String) -> String {
        if let Some(options) = &self.options {
            let reset = Color::Reset;
            let mut colored_line = line.clone();

            options.colors.iter().for_each(|(term, color)| {
                let mut find_idx = 0;

                while let Some(term_idx) = colored_line[find_idx..colored_line.len()].rfind(term) {
                    let color = color.get();
                    colored_line.insert_str(term_idx, color);
                    find_idx = term_idx + color.len() + term.len();
                    colored_line.insert_str(find_idx, reset.get());
                    find_idx = find_idx + reset.get().len();
                }
            });
            colored_line
        } else {
            line
        }
    }

    fn clear(&mut self) {
        write!(
            self.screen.borrow_mut(),
            "{}{}{}",
            termion::clear::All,
            termion::cursor::Goto(1, 1),
            termion::cursor::Hide,
        )
        .unwrap();
    }

    fn flush(&mut self) {
        self.screen.borrow_mut().flush().unwrap();
    }

    fn scroll_as_u16(&self) -> u16 {
        self.scroll as u16
    }

    fn write(&mut self) {
        write!(
            self.screen.borrow_mut(),
            "{}{}",
            termion::cursor::Goto(1, self.scroll_as_u16()),
            termion::clear::CurrentLine,
        )
        .unwrap();

        let height = self.height as usize;

        let offset = if self.current_line + height > self.lines.len() {
            self.lines.len()
        } else {
            self.current_line + height
        };

        self.lines[self.current_line..offset]
            .into_iter()
            .enumerate()
            .for_each(|(idx, line)| {
                write!(
                    self.screen.borrow_mut(),
                    "{}{}{}{}",
                    termion::cursor::Goto(1, (idx + 1) as u16),
                    termion::clear::CurrentLine,
                    self.color(line.to_string()),
                    termion::cursor::Hide
                )
                .unwrap();
            });

        let acc = (0..self.width).map(|_| "_").collect::<String>();

        write!(
            self.screen.borrow_mut(),
            "{}{}{}{}{}",
            termion::cursor::Goto(1, self.height),
            termion::clear::CurrentLine,
            termion::style::Underline,
            termion::cursor::Hide,
            acc,
        )
        .unwrap();

        write!(
            self.screen.borrow_mut(),
            "{}{}{}{}{}",
            termion::cursor::Goto(1, self.height + 2),
            termion::clear::CurrentLine,
            termion::style::Reset,
            termion::cursor::Hide,
            "Ctrl+j, k, arrow_up ,arrow_down to move, q to quit",
        )
        .unwrap();

        print!("{}", termion::style::Reset);

        self.flush();
    }

    fn scroll_down(&mut self) {
        self.scroll = if self.scroll == self.height as usize {
            self.height as usize
        } else {
            self.scroll + 1
        };

        let height = self.height as usize;

        self.current_line = if self.lines.len() < height {
            0
        } else if self.current_line == self.lines.len() - height {
            self.lines.len() - height
        } else {
            self.current_line + 1
        };

        print!("{}", termion::scroll::Up(self.scroll_as_u16()));

        self.write();
    }

    fn scroll_up(&mut self) {
        self.scroll = if self.scroll == 1 { 1 } else { self.scroll - 1 };

        self.current_line = if self.current_line == 0 {
            0
        } else {
            self.current_line - 1
        };

        print!("{}", termion::scroll::Up(self.scroll_as_u16()));

        self.write();
    }
}

pub enum Color {
    Black,
    LightBlack,
    Blue,
    LightBlue,
    Cyan,
    LightCyan,
    Green,
    LightGreen,
    Magenta,
    LightMagenta,
    Red,
    LightRed,
    White,
    LightWhite,
    Yellow,
    LightYellow,
    Reset,
}

impl Color {
    fn get(&self) -> &'static str {
        // this might seem a litle bit absurd but we don't want our user to reimport termion colors
        match self {
            Color::Black => termion::color::Black::fg_str(&termion::color::Black {}),
            Color::LightBlack => termion::color::LightBlack::fg_str(&termion::color::LightBlack),
            Color::Blue => termion::color::Blue::fg_str(&termion::color::Blue),
            Color::LightBlue => termion::color::LightBlue::fg_str(&termion::color::LightBlue),
            Color::Cyan => termion::color::Cyan::fg_str(&termion::color::Cyan),
            Color::LightCyan => termion::color::LightCyan::fg_str(&termion::color::LightCyan),
            Color::Green => termion::color::Green::fg_str(&termion::color::Green),
            Color::LightGreen => termion::color::LightGreen::fg_str(&termion::color::LightGreen),
            Color::Magenta => termion::color::Magenta::fg_str(&termion::color::Magenta),
            Color::LightMagenta => {
                termion::color::LightMagenta::fg_str(&termion::color::LightMagenta)
            }
            Color::Red => termion::color::Red::fg_str(&termion::color::Red),
            Color::LightRed => termion::color::LightRed::fg_str(&termion::color::LightRed),
            Color::White => termion::color::White::fg_str(&termion::color::White),
            Color::LightWhite => termion::color::LightWhite::fg_str(&termion::color::LightWhite),
            Color::Yellow => termion::color::Yellow::fg_str(&termion::color::Yellow),
            Color::LightYellow => termion::color::LightYellow::fg_str(&termion::color::LightYellow),
            Color::Reset => termion::color::Reset::fg_str(termion::color::Reset),
        }
    }
}
