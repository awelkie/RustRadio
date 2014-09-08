pub mod stream;
pub mod modem;
pub mod filter;

pub trait RadioBlock<A, B, I: Iterator<A>, O: Iterator<B>, P> {
    fn process(&self, input: I, params: P) -> O; //TODO get rid of "self" with UFCS
}

//This is a hack before Unified Function Call Syntax is implemented
pub struct Hack<T>;

