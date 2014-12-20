//! These blocks are for processing and manipulating streams of (almost) any type.

use super::RadioBlock;
use IteratorExtras::{IteratorExtra, MapPairs};
use IteratorExtras;

/// Splits a stream into two identical streams
#[deriving(Copy)]
pub struct Split;
pub struct SplitIter<I> {
    iterator: I,
}
impl<A: Clone, I: Iterator<A>> Iterator<(A,A)> for SplitIter<I> {
    fn next(&mut self) -> Option<(A,A)> {
        match self.iterator.next() {
            Some(a) => Some((a.clone(),a)),
            None => None,
        }
    }
}
impl<'a, A, I> RadioBlock<A, (A, A), I, SplitIter<I>> for Split
where A: Clone, I: Iterator<A> {
    fn process(&self, input: I) -> SplitIter<I> {
        SplitIter{ iterator: input }
    }
}

/// Interleaves two streams into one stream.
#[deriving(Copy)]
pub struct Interleave;
pub struct InterleaveIter<A, I> {
    iterator: I,
    other: Option<A>,
}
impl<A, I: Iterator<(A,A)>> Iterator<A> for InterleaveIter<A, I> {
    fn next(&mut self) -> Option<A> {
        match self.other.take() {
            Some(b) => Some(b),
            None => self.iterator.next().map(|(a,b)| {self.other = Some(b); a})
        }
    }
}
impl<'a, A, I> RadioBlock<(A, A), A, I, InterleaveIter<A, I>> for Interleave
where A: Clone, I: Iterator<(A, A)> {
    fn process(&self, input: I) -> InterleaveIter<A, I> {
        InterleaveIter{ iterator: input, other: None }
    }
}

#[deriving(Copy)]
pub struct DeInterleave;
impl<'a, A, I> RadioBlock<A, (A,A), I, MapPairs<'a, A, (A,A), I>> for DeInterleave
where I: Iterator<A> {
    fn process(&self, input: I) -> MapPairs<'a, A, (A,A), I> {
        input.map_pairs(|[x,y]| (x,y))
    }
}

/// Multiplies two streams.
#[deriving(Copy)]
pub struct Multiply;
pub struct MultiplyIter<I> {
    iterator: I,
}
impl<A, B, C, I> Iterator<C> for MultiplyIter<I>
where A: Mul<B,C>, I: Iterator<(A,B)> {
    fn next(&mut self) -> Option<C> {
        self.iterator.next().map(|(a,b)| a * b)
    }
}
impl<'a, A, B, C, I> RadioBlock<(A,B), C, I, MultiplyIter<I>> for Multiply
where A: Mul<B,C>, I: Iterator<(A,B)> {
    fn process(&self, input: I) -> MultiplyIter<I> {
        MultiplyIter{ iterator: input }
    }
}

/// Takes every `n`th element.
#[deriving(Copy)]
pub struct Stride {
    pub stride: uint,
}
impl<A, I> RadioBlock<A, A, I, IteratorExtras::Stride<A, I>> for Stride
where I: Iterator<A> {
    fn process(&self, input: I) -> IteratorExtras::Stride<A, I> {
        input.stride(self.stride)
    }
}
