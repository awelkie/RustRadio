extern crate IteratorExtras;

pub mod stream;
pub mod modem;
pub mod filter;

/// This is the trait that all processing blocks must follow. The block will transform
/// items of type `A` into items of type `B`. Both of these types should be tuples if more than 
/// one input or output stream is needed. The type `P` is used for parameters to the block. This
/// should be a tuple if more than one parameter is needed, or `()` if no parameters are needed.
///
/// Note that, until [UFCS](https://github.com/rust-lang/rust/issues/16293) lands, you'll need to implement RadioBlock not for your block,
/// but for `Hack<block>`. See other blocks for examples.
pub trait RadioBlock<A, B, I: Iterator<A>, O: Iterator<B>, P> {

    /// This function takes an input iterator and optional parameters and returns
    /// an output iterator. Until UFCS lands, we'll need `&self` to be in the function
    /// signature, although it shouldn't need to be used.
    fn process(&self, input: I, params: P) -> O;
}

/// This is a hack before Unified Function Call Syntax is implemented. All blocks should implement
/// `RadioBlock` for `Hack<Self>`
pub struct Hack<T>;
