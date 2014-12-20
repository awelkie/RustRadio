//! These blocks are for modulation and demodulation (both digital and analog).

use std::num::FloatMath;
use std::iter::Chain;
use std::option::IntoIter;
use num::complex::Complex;
use num::{Num, Zero, One};

use super::RadioBlock;
use IteratorExtras::{IteratorExtra, Scan1};

/// Performs analog frequency modulation.
///
/// There are no parameters. Input stream is in radians/sample. One must pre-amplify for
/// different sensitivities.
#[deriving(Copy)]
pub struct FreqMod;
pub struct FreqModIter<I, T> {
    iterator: I,
    phase: T,
}
impl<T, I> Iterator<Complex<T>> for FreqModIter<I, T>
where T: Num + FloatMath + One + Zero, I: Iterator<T> {
    fn next(&mut self) -> Option<Complex<T>> {
        self.iterator.next().map(|p| {
            self.phase = self.phase + p;
            Complex::from_polar(&One::one(), &self.phase)
        })
    }
}
impl<'r, T, I> RadioBlock<T, Complex<T>, I, Chain<IntoIter<Complex<T>>, FreqModIter<I, T>>> for FreqMod
where T: Num + FloatMath + One + Zero, I: Iterator<T> {
    fn process(&self, input: I) -> Chain<IntoIter<Complex<T>>, FreqModIter<I, T>> {
        Some(Complex::from_polar(&One::one(), &Zero::zero())).into_iter().chain(
            FreqModIter{ iterator: input, phase: Zero::zero() }
        )
    }
}

/// Calculates the phase difference between successive samples
#[deriving(Copy)]
pub struct PhaseDiffs;
impl<'r, T, I> RadioBlock<Complex<T>, T, I, Scan1<'r, Complex<T>, T, I>> for PhaseDiffs
where T: FloatMath + Clone + Num, I: Iterator<Complex<T>> {
    fn process(&self, input: I) -> Scan1<'r, Complex<T>, T, I> {
        input.scan1(|last, current| {
            let phase_diff = (current * last.conj()).arg();
            *last = current;
            Some(phase_diff)
        })
    }
}
