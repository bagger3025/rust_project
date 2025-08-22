#![warn(missing_debug_implementations, rust_2018_idioms, missing_docs)]

#[derive(Debug)]
pub struct StrSplit<'a, D> {
    remainder: Option<&'a str>,
    delimiter: D,
}

impl<'a, D> StrSplit<'a, D> {
    pub fn new(haystack: &'a str, delimiter: D) -> Self {
        Self {
            remainder: Some(haystack),
            delimiter,
        }
    }
}

pub trait Delimiter {
    fn find_next(&self, s: &str) -> Option<(usize, usize)>;
}

impl<'a, D> Iterator for StrSplit<'a, D>
where
    D: Delimiter,
{
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        /* let Some(remainder) = &mut self.remainder */

        /* self.remainder: Option<T> -> T, remainder becomes T (with copied), and ref mut T = &mut T */
        // let ref mut remainder = self.remainder?;

        /* self.remainder.as_mut(): Option<&mut T>, remainder becomes &mut T (with copied) */
        // let remainder = self.remainder.as_mut()?;
        if let Some(ref mut remainder) = self.remainder {
            if let Some((delim_start, delim_end)) = self.delimiter.find_next(remainder) {
                let until_delimiter = &remainder[..delim_start];
                *remainder = &remainder[delim_end..];
                Some(until_delimiter)
            } else {
                self.remainder.take()
            }
        } else {
            None
        }
    }

    // fn next(&mut self) -> option<self::item> {
    //     let some(remainder) = self.remainder else {
    //         return none;
    //     };

    //     if let some(next_delim) = remainder.find(self.delimiter) {
    //         let until_delimiter = &remainder[..next_delim];
    //         self.remainder = some(&remainder[(next_delim + self.delimiter.len())..]);
    //         some(until_delimiter)
    //     } else if remainder.is_empty() {
    //         self.remainder = none;
    //         some("")
    //     } else {
    //         let rest = self.remainder;
    //         self.remainder = none;
    //         rest
    //     }
    // }
}

impl Delimiter for &String {
    fn find_next(&self, s: &str) -> Option<(usize, usize)> {
        s.find(*self).map(|start| (start, start + self.len()))
    }
}

impl Delimiter for &str {
    fn find_next(&self, s: &str) -> Option<(usize, usize)> {
        s.find(self).map(|start| (start, start + self.len()))
    }
}

impl Delimiter for char {
    fn find_next(&self, s: &str) -> Option<(usize, usize)> {
        s.char_indices()
            .find(|(_, c)| c == self)
            .map(|(start, _)| (start, start + self.len_utf8()))
    }
}

pub fn until_char(s: &str, c: char) -> &str {
    StrSplit::new(s, c)
        .next()
        .expect("Strsplit always gives at least one result")
}

#[test]
fn until_char_test() {
    assert_eq!(until_char("hello world", 'o'), "hell");
}

#[test]
fn it_works() {
    let haystack = "a b c d e";
    let letters = StrSplit::new(haystack, " ");
    assert!(letters.eq(vec!["a", "b", "c", "d", "e"].into_iter()));
}

#[test]
fn works_when_multiple_delimiter() {
    let haystack = "a  b c d e";
    let letters = StrSplit::new(haystack, " ");
    assert_eq!(
        letters.collect::<Vec<_>>(),
        vec!["a", "", "b", "c", "d", "e"]
    );
}

#[test]
fn last_delimiter() {
    let haystack = "a b c d ";
    let letters = StrSplit::new(haystack, " ");
    assert_eq!(letters.collect::<Vec<_>>(), vec!["a", "b", "c", "d", ""]);
}
