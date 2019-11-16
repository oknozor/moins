use std::cell::RefCell;
use std::io::Write;
use std::io::{stdin, stdout, Stdout};
use termion::event::Key;
use termion::input::MouseTerminal;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::raw::RawTerminal;
use termion::screen::AlternateScreen;

type Terminal = RefCell<MouseTerminal<AlternateScreen<RawTerminal<Stdout>>>>;

pub struct Pager<'a> {
    lines: Vec<&'a str>,
    height: u16,
    pub current_line: usize,
    pub scroll: usize,
    pub screen: Terminal,
}

impl<'a> Pager<'a> {
    pub fn run(content: String) {
        let stdout = stdout().into_raw_mode().unwrap();
        let screen = MouseTerminal::from(AlternateScreen::from(stdout));
        let screen = RefCell::new(screen);

        let mut pager = Pager::new(&content, screen);

        let stdin = stdin();

        pager.clear();
        pager.write();

        for c in stdin.keys() {
            // Input
            match c.unwrap() {
                Key::Char('q') => break,
                Key::Down | Key::Char('j') => pager.scroll_down(),
                Key::Up | Key::Char('k') => pager.scroll_up(),
                _ => (),
            }
        }
    }

    fn new(content: &'a String, screen: Terminal) -> Self {
        let size = termion::terminal_size().unwrap();
        let width = size.0 as usize;
        let lines: Vec<&str> = content.split("\n").collect();

        let mut sized_lines = vec![];

        lines.iter().for_each(|line| {
            if line.len() > width {
                sized_lines.push(&line[0..width]);
                sized_lines.push(&line[width..line.len()]);
            } else {
                sized_lines.push(line);
            }
        });

        Pager {
            lines,
            height: size.1,
            current_line: 0,
            scroll: size.1 as usize,
            screen,
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

    pub fn write(&mut self) {
        let height = self.height as usize;

        self.lines[self.current_line..self.current_line + height]
            .into_iter()
            .enumerate()
            .for_each(|(idx, line)| {
                write!(
                    self.screen.borrow_mut(),
                    "{}{}{}{}",
                    termion::cursor::Goto(1, (idx + 1) as u16),
                    termion::clear::CurrentLine,
                    line,
                    termion::cursor::Hide
                )
                .unwrap();
            });
        self.flush();
    }

    pub fn scroll_down(&mut self) {
        self.scroll = if self.scroll == self.height as usize {
            self.height as usize
        } else {
            self.scroll + 1
        };

        let height = self.height as usize;

        if self.current_line == self.lines.len() - height {
            self.current_line = self.lines.len() - height;
        } else {
            self.current_line = self.current_line + 1;
        };

        print!("{}", termion::scroll::Up(self.scroll_as_u16()));
        write!(
            self.screen.borrow_mut(),
            "{}{}",
            termion::cursor::Goto(1, self.scroll_as_u16()),
            termion::clear::CurrentLine,
        )
        .unwrap();

        self.write();
    }

    pub fn scroll_up(&mut self) {
        self.scroll = if self.scroll == 1 { 1 } else { self.scroll - 1 };

        self.current_line = if self.current_line == 0 {
            0
        } else {
            self.current_line - 1
        };

        print!("{}", termion::scroll::Up(self.scroll_as_u16()));

        write!(
            self.screen.borrow_mut(),
            "{}{}",
            termion::cursor::Goto(1, self.scroll_as_u16()),
            termion::clear::CurrentLine,
        )
        .unwrap();

        self.write();
    }
}
