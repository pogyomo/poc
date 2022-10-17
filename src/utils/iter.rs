use std::mem::{swap, replace};

pub trait MorePeekable: Iterator + Sized {
    fn more_peekable(self) -> MorePeekableIter<Self> {
        MorePeekableIter::new(self)
    }
}

impl<I: Iterator> MorePeekable for I {}

pub struct MorePeekableIter<I: Iterator> {
    iter: I,
    curr_item: Option<I::Item>,
    next_item: Option<I::Item>,
}

impl<I: Iterator> MorePeekableIter<I> {
    pub fn new(mut iter: I) -> MorePeekableIter<I> {
        let curr_item = iter.next();
        let next_item = iter.next();
        MorePeekableIter { iter, curr_item, next_item }
    }
}

impl<I: Iterator> Iterator for MorePeekableIter<I> {
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        swap(&mut self.curr_item, &mut self.next_item);
        replace(&mut self.next_item, self.iter.next())
    }
}

impl<I: Iterator> MorePeekableIter<I> {
    pub fn peek_curr(&self) -> Option<&I::Item> {
        self.curr_item.as_ref()
    }

    pub fn peek_next(&self) -> Option<&I::Item> {
        self.next_item.as_ref()
    }
}
