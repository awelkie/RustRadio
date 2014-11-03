//! These blocks are for digital filtering.

use std::num::Zero;
use std::collections::RingBuf;
use std::iter::AdditiveIterator;

use super::RadioBlock;
use super::IteratorExtras::{IteratorExtra};

/// Applies an FIR filter.
///
/// Parameter is a slice containing the filter taps. The first tap
/// multiplies the most recent sample, and the last tap multiplies
/// the earliest sample in the history. This doesn't matter for symmetric
/// filters.
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
            for (i, m) in self.filter.iter().map(|a| x * *a).enumerate() {
                self.buff[i] = m + self.buff[i + 1];
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

/// Polyphase Rational Resampler
///
/// This block resamples the incoming samples at a rational factor. It
/// upsamples the signal, applies the supplied FIR Filter, then downsamples.
///
/// The taps are in the same order as the `FilterFIR`, meaning the first tap (at
/// index 0) multiplies the most recent sample
pub struct RationalResampler<'b, B: 'b>{
    pub up: uint,
    pub down: uint,
    pub taps: &'b [B],
}

pub struct RationalResamplerIter<A, B, I: Iterator<A>> {
    up: uint,
    down: uint,
    filter_length: uint,
    filters: Vec<Vec<B>>,
    filter_idx: uint,
    sample_history: RingBuf<A>,
    iterator: I,
}

impl<A, B, C, I> Iterator<C> for RationalResamplerIter<A, B, I>
where A: Zero, B: Mul<A,C>, C: Zero, I: Iterator<A> {
    fn next(&mut self) -> Option<C> {
        if self.sample_history.is_empty() {
            // start off with all zeros and the first element
            self.sample_history.reserve_exact(self.filter_length);
            for _ in range(0u, self.filter_length - 1) {
                self.sample_history.push_front(Zero::zero());
            }
            match self.iterator.next() {
                None => return None,
                Some(x) => self.sample_history.push_front(x),
            }
        }

        // Get new samples, if needed
        while self.filter_idx >= self.up {
            self.filter_idx -= self.up;
            self.sample_history.pop();
            match self.iterator.next() {
                None => return None,
                Some(x) => self.sample_history.push_front(x)
            }
        }

        // Save the current filter index, and then increment for next time
        let current_filter_idx = self.filter_idx;
        self.filter_idx += self.down;

        // Correlate the most recent samples against the current FIR filter
        Some(self.filters[current_filter_idx].iter().zip(self.sample_history.iter())
            .fold(Zero::zero(), |sum: C, (a, b)| sum + a * *b))
    }
}

impl<'b, A, B, C, I> RadioBlock<A, C, I, RationalResamplerIter<A, B, I>> for RationalResampler<'b, B>
where A: Zero, B: Mul<A,C> + Clone, C: Zero, I: Iterator<A> {
    fn process(&self, input: I) -> RationalResamplerIter<A, B, I> {

        // Split the given FIR filter into smaller filters
        let mut filters: Vec<Vec<B>> = Vec::new();
        for i in range(0, self.up) {
            filters.push(self.taps.iter().map(|x| x.clone()).skip(i).stride(self.up).collect());
        }

        RationalResamplerIter {
            up: self.up,
            down: self.down,
            filter_length: filters[0].len(),
            filters: filters,
            filter_idx: 0,
            sample_history: RingBuf::new(),
            iterator: input,
        }
    }
}

pub trait WindowFunction {
    fn time_domain_taps(&self, num_taps: uint) -> Vec<f32>;
}

pub struct HammingWindow;
impl WindowFunction for HammingWindow {
    fn time_domain_taps(&self, num_taps: uint) -> Vec<f32> {
        let alpha = 0.54;
        let beta = 1.0 - alpha;
        let tau: f32 = Float::two_pi();
        Vec::from_fn(num_taps, |i| {
            let n = i - (num_taps - 1) / 2;
            alpha - beta * (tau * (n as f32) / ((num_taps as f32) - 1.0)).cos()
            })
    }
}

/// FIXME actually implement this
fn n_taps_needed(transition_width: f32) -> uint {
    101
}

pub enum NumTapsSpecifier {
    NumTaps(uint),
    TransitionWidth(f32),
}

/// Generates the taps for a low-pass filter
///
/// `window_type` is the window we use to generate the taps
/// `bandwidth` is the normalized bandwidth of the filter, which is the
///             cutoff frequency divided by the sampling frequency
/// `num_taps` is either `NumTaps(n)`, which specifies the number of taps
///            directly, or `TransitionWidth(w)`, which gives the desired
///            transition width (normalized, like `bandwidth`), and the
///            number of taps is calculated from this.
pub fn low_pass_filter_taps<W: WindowFunction>(window_type: W,
                                               bandwidth: f32,
                                               num_taps: NumTapsSpecifier) -> Vec<f32> {
    let n_taps = match num_taps {
        NumTaps(n) => n,
        TransitionWidth(width) => n_taps_needed(width),
    };

    // start out with window function
    let mut taps = window_type.time_domain_taps(n_taps);

    // multiply by sinc
    for (idx, tap) in taps.iter_mut().enumerate() {
        // convert from vector index to time index
        let time_idx = idx - (n_taps - 1) / 2;
        *tap *= if time_idx == 0 {
                2.0 * bandwidth
            } else {
                (time_idx as f32 * Float::two_pi() * bandwidth).sin() /
                    (time_idx as f32 * Float::pi())
            }
    }

    // normalize
    let sum = taps.iter().map(|&x| x).sum();
    for tap in taps.iter_mut() {
        *tap /= sum;
    }

    return taps;
}
