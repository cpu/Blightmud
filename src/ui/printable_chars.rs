use std::str::Chars;
extern crate vte;
use vte::{Parser, Perform};

pub(crate) trait PrintableCharsIterator<'a> {
    fn printable_chars(&self) -> PrintableChars<'a>;
}

impl<'a> PrintableCharsIterator<'a> for &'a str {
    fn printable_chars(&self) -> PrintableChars<'a> {
        PrintableChars {
            iter: self.chars(),
            parser: Parser::new(),
            performer: Performer::new(),
        }
    }
}

struct Performer {
    c: Option<char>,
}

impl Performer {
    fn new() -> Self {
        Performer { c: None }
    }
}

impl Perform for Performer {
    fn print(&mut self, c: char) {
        self.c = Some(c)
    }
}

#[must_use = "iterators are lazy and do nothing unless consumed"]
pub(crate) struct PrintableChars<'a> {
    pub(super) iter: Chars<'a>,
    parser: Parser,
    performer: Performer,
}

impl<'a> Iterator for PrintableChars<'a> {
    type Item = char;

    #[inline]
    fn next(&mut self) -> Option<char> {
        let next = self.iter.next();
        match next {
            Some(c) => {
                self.parser.advance(&mut self.performer, c as u8);
                self.performer.c.take()
            }
            None => None,
        }
    }
}
