extern crate num;
use self::num::complex::Complex;
use std::num::Zero;
use std::num::One;

use super::{RadioBlock, Hack};

pub struct FreqMod;
struct FreqModiter<T, I> {
    iterator: I,
    phase: T,
    sensitivity: T,
}
impl<T: Num + FloatMath, I: Iterator<T>> Iterator<Complex<T>> for FreqModiter<T, I> {
    fn next(&mut self) -> Option<Complex<T>> {
        self.iterator.next().map(|f| {
            self.phase = self.phase + f * self.sensitivity;
            self.phase = self.phase % Float::two_pi();
            Complex::from_polar(&One::one(), &self.phase)
        })
    }
}
#[allow(visible_private_types)]
impl<T: Num + FloatMath, I: Iterator<T>> RadioBlock<T, Complex<T>, I, FreqModiter<T,I>, T> for Hack<FreqMod> {
    fn process(&self, input: I, params: T) -> FreqModiter<T,I> {
        FreqModiter {
            iterator: input,
            phase: Zero::zero(),
            sensitivity: params,
        }
    }
}
