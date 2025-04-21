#![allow(unused)]

use crate::book::Book;

pub trait Reader<'reader> {
    fn borrow(&mut self, book: &'reader Book);
}

#[derive(Default, Debug)]
pub struct GenericReader<'reader> {
    books: Option<&'reader Book>,
}
impl<'reader> Reader<'reader> for GenericReader<'reader> {
    fn borrow(&mut self, book: &'reader Book) {
        self.books.replace(book);
    }
}

pub enum Proxy<'t, T> {
    None,
    Some(&'t mut T),
}

pub struct Library<'reader, R> {
    readers: Vec<Proxy<'reader, R>>,
}

impl<'reader, R: Reader<'reader> + 'reader> Library<'reader, R> {
    pub fn new(readers: impl IntoIterator<Item = &'reader mut R>) -> Self {
        Self {
            readers: Vec::from_iter(readers.into_iter().map(Proxy::Some)),
        }
    }
}
