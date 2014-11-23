//! These blocks are for modulation and demodulation (both digital and analog).

use std::num::FloatMath;
use std::iter::{Scan, Chain};
use std::option::Item;
use num::complex::Complex;
use num::{Num, Zero, One};

use super::RadioBlock;
use IteratorExtras::{IteratorExtra, Scan1};

/// Performs analog frequency modulation.
///
/// There are no parameters. Input stream is in radians/sample. One must pre-amplify for
/// different sensitivities.
pub struct FreqMod;
impl<'r, T, I> RadioBlock<T, Complex<T>, I, Chain<Item<Complex<T>>, Scan<'r, T, Complex<T>, I, T>>> for FreqMod
where T: Num + FloatMath + One + Zero, I: Iterator<T> {
    fn process(&self, input: I) -> Chain<Item<Complex<T>>, Scan<'r, T, Complex<T>, I, T>> {
        Some(Complex::from_polar(&One::one(), &Zero::zero())).into_iter().chain(
            input.scan(Zero::zero(), |phase: &mut T, f| {
                *phase = *phase + f;
                Some(Complex::from_polar(&One::one(), phase))
        }))
    }
}

/// Calculates the phase difference between successive samples
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
