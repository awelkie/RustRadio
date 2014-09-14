//! These blocks are for modulation and demodulation (both digital and analog).

extern crate num;
use self::num::complex::Complex;
use std::num::Zero;
use std::num::One;
use std::iter::{Scan, Fuse};

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

pub struct PhaseDiffs;
impl<'r, T: FloatMath + Clone, I: Iterator<Complex<T>>> RadioBlock<Complex<T>, T, I, Scan<'r, Complex<T>, T, Fuse<I>, Complex<T>>, ()> for Hack<PhaseDiffs> {
    fn process(&self, input: I, _: ()) -> Scan<'r, Complex<T>, T, Fuse<I>, Complex<T>> {
        let mut fused_input = input.fuse();
        let first_sample = fused_input.next().unwrap_or(Zero::zero());
        fused_input.scan(first_sample, |last, current| {
            let phase_diff = (current * last.conj()).arg();
            *last = current;
            Some(phase_diff)
        })
    }
}
