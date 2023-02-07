use std::str::{CharIndices, Chars};
extern crate vte;
use vte::{Parser, Perform};

pub(crate) trait PrintableCharsIterator<'a> {
    fn printable_chars(&self) -> PrintableChars<'a>;
    fn printable_char_indices(&self) -> PrintableCharIndices<'a>;
}

impl<'a> PrintableCharsIterator<'a> for &'a str {
    fn printable_chars(&self) -> PrintableChars<'a> {
        PrintableChars {
            iter: self.chars(),
            parser: Parser::new(),
            performer: Performer::new(),
        }
    }

    fn printable_char_indices(&self) -> PrintableCharIndices<'a> {
        PrintableCharIndices {
            iter: self.char_indices(),
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
    iter: Chars<'a>,
    parser: Parser,
    performer: Performer,
}

impl<'a> Iterator for PrintableChars<'a> {
    type Item = char;

    #[inline]
    fn next(&mut self) -> Option<char> {
        let mut next = self.iter.next();

        while let Some(c) = next {
            self.parser.advance(&mut self.performer, c as u8);
            if let Some(pc) = self.performer.c.take() {
                return Some(pc);
            } else {
                next = self.iter.next();
            }
        }

        return None;
    }
}

#[must_use = "iterators are lazy and do nothing unless consumed"]
pub(crate) struct PrintableCharIndices<'a> {
    iter: CharIndices<'a>,
    parser: Parser,
    performer: Performer,
}

impl<'a> Iterator for PrintableCharIndices<'a> {
    type Item = (usize, char);

    #[inline]
    fn next(&mut self) -> Option<(usize, char)> {
        let mut next = self.iter.next();

        while let Some((offset, c)) = next {
            self.parser.advance(&mut self.performer, c as u8);
            if let Some(c) = self.performer.c.take() {
                return Some((offset, c));
            } else {
                next = self.iter.next();
            }
        }

        return None;
    }
}
