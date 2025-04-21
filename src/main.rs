#![allow(unused)]

use lifetime_wut_::book::Book;
use std::iter::repeat_n;

fn prob_a() {
    use lifetime_wut_::prob_a::*;
    let books = vec![Book, Book, Book];
    let mut readers = Vec::from_iter(repeat_n((), 5).map(|_| GenericReader::default()));
    {
        let library = Library::new(&mut readers);
    }
    dbg!(&readers);
}

fn prob_b() {
    use lifetime_wut_::prob_b::*;
    let books = vec![Book, Book, Book];
    let mut readers = Vec::from_iter(repeat_n((), 5).map(|_| GenericReader::default()));
    {
        let library = Library::new(&mut readers);
    }
    dbg!(&readers);
}

fn prob_c() {
    use lifetime_wut_::prob_c::*;
    let books = vec![Book, Book, Book];
    let mut readers = Vec::from_iter(repeat_n((), 5).map(|_| GenericReader::default()));
    {
        let library = Library::new(&mut readers);
    }
    dbg!(&readers);
}

fn main() {
    prob_a();
    prob_b();
    prob_c();
}
