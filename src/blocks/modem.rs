//! These blocks are for modulation and demodulation (both digital and analog).

extern crate num;
use self::num::complex::Complex;
use std::num::Zero;
use std::num::One;

use super::{RadioBlock, Hack};

/// Performs analog frequency modulation.
///
/// There are no parameters. Input stream is in radians/sample. One must pre-amplify for
/// different sensitivities.
pub struct FreqMod;
struct FreqModiter<T, I> {
    iterator: I,
    phase: T,
}
impl<T: Num + FloatMath, I: Iterator<T>> Iterator<Complex<T>> for FreqModiter<T, I> {
    fn next(&mut self) -> Option<Complex<T>> {
        self.iterator.next().map(|f| {
            self.phase = (self.phase + f) % Float::two_pi();
            Complex::from_polar(&One::one(), &self.phase)
        })
    }
}
#[allow(visible_private_types)]
impl<T: Num + FloatMath, I: Iterator<T>> RadioBlock<T, Complex<T>, I, FreqModiter<T,I>, ()> for Hack<FreqMod> {
    fn process(&self, input: I, _: ()) -> FreqModiter<T,I> {
        FreqModiter {
            iterator: input,
            phase: Zero::zero(),
        }
    }
}
