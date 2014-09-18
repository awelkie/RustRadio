//! These blocks are for modulation and demodulation (both digital and analog).

extern crate num;
use self::num::complex::Complex;
use std::num::Zero;
use std::num::One;
use std::iter::{Scan, Fuse, Chain};
use std::option;

use super::{RadioBlock, Hack};

/// Performs analog frequency modulation.
///
/// There are no parameters. Input stream is in radians/sample. One must pre-amplify for
/// different sensitivities.
pub struct FreqMod;
impl<'r, T: Num + FloatMath, I: Iterator<T>> RadioBlock<T, Complex<T>, I, Chain<option::Item<Complex<T>>, Scan<'r, T, Complex<T>, I, T>>, ()> for Hack<FreqMod> {
    fn process(&self, input: I, _: ()) -> Chain<option::Item<Complex<T>>, Scan<'r, T, Complex<T>, I, T>> {
        Some(Complex::from_polar(&One::one(), &Zero::zero())).into_iter().chain(
            input.scan(Zero::zero(), |phase: &mut T, f| {
                *phase = *phase + f;
                Some(Complex::from_polar(&One::one(), phase))
        }))
    }
}

/// Calculates the phase difference between successive samples
pub struct PhaseDiffs;
impl<'r, T: FloatMath + Clone, I: Iterator<Complex<T>>> RadioBlock<Complex<T>, T, I, Scan<'r, Complex<T>, T, Fuse<I>, Complex<T>>, ()> for Hack<PhaseDiffs> {
    fn process(&self, input: I, _: ()) -> Scan<'r, Complex<T>, T, Fuse<I>, Complex<T>> {
        // Note that this is really just a call to scan1, i.e. scan without an initial value.
        let mut fused_input = input.fuse();
        let first_sample = fused_input.next().unwrap_or(Zero::zero());
        fused_input.scan(first_sample, |last, current| {
            let phase_diff = (current * last.conj()).arg();
            *last = current;
            Some(phase_diff)
        })
    }
}
