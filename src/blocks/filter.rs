//! These blocks are for digital filtering.

use std::num::Zero;

use super::RadioBlock;

/// Applies an FIR filter.
///
/// Parameter is a slice containing the filter taps.
pub struct FilterFIR<'b, B: 'b> {
    pub taps: &'b [B],
}
pub struct FilterFIRiter<A, B, C, I> {
    filter: Vec<B>,
    buff: Vec<C>, //needs to be one larger than filter, with a 0 at the end;
    iterator: I,
}

impl<B, C: Mul<C,C> + Zero + Clone, A: Mul<B,C>, I: Iterator<A>> Iterator<C> for FilterFIRiter<A,B,C,I> {
    fn next(&mut self) -> Option<C> {
        self.iterator.next().map(|x| {
            for (i, m) in self.filter.iter().rev().map(|a| x * *a).enumerate() {
                *(self.buff.get_mut(i)) = m + self.buff[i + 1];
            }
            self.buff[0].clone()
        })
    }
}

impl<'b, A, B, C, I> RadioBlock<A, C, I, FilterFIRiter<A,B,C,I>> for FilterFIR<'b, B>
where A: Mul<B,C>, B: Clone, C: Mul<C,C> + Zero + Clone, I: Iterator<A>{
    fn process(&self, input: I) -> FilterFIRiter<A,B,C,I> {
        FilterFIRiter {
            filter: self.taps.to_vec(),
            buff: Vec::from_elem(self.taps.len() + 1, Zero::zero()),
            iterator: input
        }
    }
}
