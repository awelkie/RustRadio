extern crate IteratorExtras;

pub mod stream;
pub mod modem;
pub mod filter;

/// This is the trait that all processing blocks must follow. The block will transform
/// items of type `A` into items of type `B`. Both of these types should be tuples if more than
/// one input or output stream is needed.
///
pub trait RadioBlock<A, B, I: Iterator<A>, O: Iterator<B>> {

    /// This function takes the input iterator and transforms it to
    /// another iterator using any parameters found in the block object.
    fn process(&self, input: I) -> O;
}
