use std::iter;
use std::num::Zero;
use std::option;

pub trait RadioBlock<A, B, I: Iterator<A>, O: Iterator<B>, P> {
    fn process(&self, input: I, params: P) -> O; //TODO get rid of "self" with UFCS
}

//This is a hack before Unified Function Call Syntax is implemented
pub struct Hack<T>;

pub struct Identity;
impl<'a, A: Clone, I: Iterator<A>> RadioBlock<A, A, I, iter::Map<'a, A, A, I>, ()> for Hack<Identity> {
    fn process(&self, input: I, _: ()) -> iter::Map<'a, A, A, I> {
        input.map(|x| x.clone() )
    }
}

pub struct Split;
impl<'a, A: Clone, I: Iterator<A>> RadioBlock<A, (A, A), I, iter::Map<'a, A, (A, A), I>, ()> for Hack<Split> {
    fn process(&self, input: I, _: ()) -> iter::Map<'a, A, (A, A), I> {
        input.map(|x| (x.clone(), x.clone()))
    }
}

pub struct Interleave;
type InterleaveOutput<'a, A, I> = iter::FlatMap<'a,(A,A),I,iter::Chain<option::Item<A>,option::Item<A>>>;
#[allow(visible_private_types)]
impl<'a, A: Clone, I: Iterator<(A, A)>> RadioBlock<(A, A), A, I, InterleaveOutput<'a, A, I>, ()> for Hack<Interleave> {
    fn process(&self, input: I, _: ()) -> iter::FlatMap<'a,(A,A),I,iter::Chain<option::Item<A>,option::Item<A>>> {
        input.flat_map(|(l, r)| Some(l.clone()).move_iter().chain(Some(r.clone()).move_iter()))
    }
}

pub struct Multiply;
impl<'a, B, C, A: Mul<B,C>, I: Iterator<(A,B)>> RadioBlock<(A,B), C, I, iter::Map<'a, (A,B), C, I>, ()> for Hack<Multiply> {
    fn process(&self, input: I, _: ()) -> iter::Map<'a, (A,B), C, I> {
        input.map(|(a,b)| a * b)
    }
}

pub struct Stride;
struct Strider<I> {
    iterator: I,
    stride: uint
}
impl<A, I: Iterator<A>> Iterator<A> for Strider<I> {
    fn next(&mut self) -> Option<A> {
        self.iterator.nth(self.stride)
    }
}
#[allow(visible_private_types)]
impl<A, I: Iterator<A>> RadioBlock<A, A, I, Strider<I>, uint> for Hack<Stride> {
    fn process(&self, input: I, stride: uint) -> Strider<I> {
        Strider{ iterator: input, stride: stride }
    }
}

pub struct FilterFIR;
struct FilterFIRiter<A, B, C, I> {
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
#[allow(visible_private_types)]
impl<'b, B: Clone, C: Mul<C,C> + Zero + Clone, A: Mul<B,C>, I: Iterator<A>> RadioBlock<A, C, I, FilterFIRiter<A,B,C,I>, &'b [B]> for Hack<FilterFIR> {
    fn process(&self, input: I, params: &'b [B]) -> FilterFIRiter<A,B,C,I> {
        FilterFIRiter {
            filter: Vec::from_slice(params),
            buff: Vec::from_elem(params.len() + 1, Zero::zero()),
            iterator: input
        }
    }
}
