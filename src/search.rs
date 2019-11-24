use termion::cursor::Goto;

#[derive(Debug)]
pub struct Search {
    inner: Vec<InlineMatches>,
    current_line: usize,
    size: Option<usize>,
    pub term_len: u16,
}

#[derive(Debug)]
pub struct InlineMatches {
    pub term_line_idx: u16,
    pub current_hl: usize,
    pub matches: Vec<usize>,
}

impl Search {
    pub fn new(search_term: &str, lines: &Vec<&str>) -> Search {
        let inner: Vec<InlineMatches> = lines
            .iter()
            .enumerate()
            .map(|(idx, line)| {
                let on_line_indices: Vec<usize> = line
                    .match_indices(search_term)
                    .into_iter()
                    .map(|(term_idx, _)| term_idx)
                    .collect();

                if on_line_indices.is_empty() {
                    None
                } else {
                    Some(InlineMatches::new((idx + 1) as u16, on_line_indices))
                }
            })
            .filter(|opt| opt.is_some())
            .map(|matches| matches.unwrap())
            .collect();

            Search {
                inner,
                current_line: 0,
                size: None,
                term_len: search_term.len() as u16
            }
    }
    pub fn next(&mut self) -> Option<Goto> {
        if let Some(next) = self.inner[self.current_line].next() {
            Some(next)
        } else if self.current_line + 1 < self.inner.len() {
            self.current_line += 1;
            self.next()
        } else {
            None
        }
    }

    pub fn prev(&mut self) -> Option<Goto> {
        if let Some(prev) = self.inner[self.current_line].prev() {
            Some(prev)
        } else if self.current_line > 0 {
            self.current_line -= 1;
            self.prev()
        } else {
            None
        }
    }

    pub fn first(&mut self) -> Goto {
        self.current_line = 0;
        self.inner[0].first()
    }

    pub fn last(&mut self) -> Goto {
        self.current_line = self.inner.len() - 1;
        self.inner[self.current_line].last()
    }

    fn size(&mut self) -> usize {
        if let Some(size) = self.size {
            size
        } else {
            let size = self
                .inner
                .iter()
                .fold(0, |acc, inner| acc + inner.matches.len());
            size
        }
    }

    pub fn is_empty(&mut self) -> bool {
        self.size() == 0
    }
}

impl InlineMatches {
    pub fn new(term_line_idx: u16, matches: Vec<usize>) -> InlineMatches {
        InlineMatches {
            current_hl: 0,
            term_line_idx,
            matches,
        }
    }

    pub fn next(&mut self) -> Option<Goto> {
        if self.current_hl > self.matches.len() - 1 {
            return None;
        } else {
            self.current_hl += 1;
        }

        let hl = if self.current_hl == 1 {
            self.matches[0]
        } else {
            self.matches[self.current_hl]
        };

        Some(Goto(hl as u16, self.term_line_idx))
    }

    pub fn prev(&mut self) -> Option<Goto> {
        if self.current_hl == 0 {
            return None;
        } else {
            self.current_hl -= 1;
        }

        let hl = self.matches[self.current_hl];

        Some(Goto(hl as u16, self.term_line_idx))
    }

    pub fn first(&mut self) -> Goto {
        self.current_hl = 0;
        Goto((self.matches[0] + 1) as u16, self.term_line_idx)
    }

    fn last(&mut self) -> Goto {
        self.current_hl = self.matches.len() - 1;
        Goto(
            (self.matches[self.current_hl] + 1) as u16,
            self.term_line_idx,
        )
    }
}

impl std::ops::Index<usize> for Search {
    type Output = InlineMatches;

    fn index(&self, ix: usize) -> &Self::Output {
        self.inner.index(ix)
    }
}

impl std::ops::IndexMut<usize> for Search {
    fn index_mut(&mut self, ix: usize) -> &mut InlineMatches {
        self.inner.index_mut(ix)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn should_go_first_inline() {
        let lines: &Vec<&str> = &vec![
            "Le cœur sur l’arbre vous n’aviez qu’à le cueillir,",
            "Sourire et rire, rire et douceur d’outre-sens.",
        ];

        let mut result = Search::new("rire", lines);

        assert_eq!(result.first().0, 4);
        assert_eq!(result.first().1, 2);
    }

    #[test]
    fn should_go_last_inline() {
        let lines: &Vec<&str> = &vec![
            "Les jours comme des doigts repliant leurs phalanges.",
            "Les fleurs sont desséchées, les graines sont perdues,",
            "La canicule attend les grandes gelées blanches.",
        ];

        let mut result = Search::new("les", lines);

        assert_eq!(result.last().0, 20);
        assert_eq!(result.last().1, 3);
    }

    #[test]
    fn should_go_next() {
        let lines: &Vec<&str> = &vec![
            "Au loin, geint une belle qui voudrait lutter",
            "Et qui ne peut, couchée au pied de la colline.",
            "Et que le ciel soit misérable ou transparent",
            "On ne peut la voir sans l’aimer.",
        ];

        let mut result = Search::new("peut", lines);

        let next = result.next().unwrap();
        assert_eq!((next.0, next.1), (10, 2));

        let next = result.next().unwrap();
        assert_eq!((next.0, next.1), (6, 4));

        let next = result.next();
        assert!(!next.is_some());
    }

    #[test]
    fn should_go_prev() {
        let lines: &Vec<&str> = &vec![
            "Au loin, geint une belle qui voudrait lutter",
            "Et qui ne peut, couchée au pied de la colline.",
            "Et que le ciel soit misérable ou transparent",
            "On ne peut la voir sans l’aimer.",
        ];

        let mut result = Search::new("peut", lines);

        println!("{:?}", result);
        let prev = result.prev();
        assert!(!prev.is_some());

        let _ = result.next().unwrap();
        let _ = result.next().unwrap();
        let _ = result.next();

        let prev = result.prev().unwrap();
        assert_eq!((prev.0, prev.1), (6, 4));
    }
}
