use std::cell::RefCell;
use std::io::Write;
use std::io::{
    stdin,
    stdout,
    Stdout,
};
use termion::event::Key;
use termion::input::{
    MouseTerminal,
    TermRead,
};
use termion::raw::{
    IntoRawMode,
    RawTerminal,
};
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

        Pager {
            lines,
            scroll,
            screen,
            current_line: 0,
            height: size.1,
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
                    line,
                    termion::cursor::Hide
                )
                .unwrap();
            });
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
