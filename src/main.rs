#![allow(unused)]

use lifetime_wut_::book::Book;
use std::iter::repeat_n;

fn prob_a0() {
    use lifetime_wut_::prob_a0::*;
    let books = vec![Book, Book, Book];
    let mut readers = Vec::from_iter(repeat_n((), 5).map(|_| GenericReader::default()));
    {
        let library = Library::new(&mut readers);
    }
    dbg!(&readers);
}

fn prob_a1() {
    use lifetime_wut_::prob_a1::*;
    let books = vec![Book, Book, Book];
    let mut readers = Vec::from_iter(repeat_n((), 5).map(|_| GenericReader::default()));
    {
        let library = Library::new(&mut readers);
    }
    dbg!(&readers);
}

fn prob_b0() {
    use lifetime_wut_::prob_b0::*;
    let books = vec![Book, Book, Book];
    let mut readers = Vec::from_iter(repeat_n((), 5).map(|_| GenericReader::default()));
    {
        let library = Library::new(&mut readers);
    }
    dbg!(&readers);
}

fn prob_b1() {
    use lifetime_wut_::prob_b1::*;
    let books = vec![Book, Book, Book];
    let mut readers = Vec::from_iter(repeat_n((), 5).map(|_| GenericReader::default()));
    {
        let library = Library::new(&mut readers);
    }
    dbg!(&readers);
}

fn main() {
    prob_a0();
    prob_a1();
    prob_b0();
    prob_b1();
}
