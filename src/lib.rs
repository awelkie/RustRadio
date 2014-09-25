#![feature(unboxed_closures)]
#![feature(overloaded_calls)]
#![feature(associated_types)]

#![feature(macro_rules)]
#![feature(globs)]
#[macro_escape]

/// The processing blocks, broken out into submodules
pub mod blocks;
/// The buffers that exist between blocks
pub mod buffers;
/// Different sources of samples
pub mod sources;

pub static DEFAULT_BUFFER_SIZE: uint = 2048;

/// Simplifies connecting blocks with fixed buffers.
///
/// Right now, only blocks that operate on 1 or 2 streams is supported. More
/// input/output streams should be easy to add.
#[macro_export]
macro_rules! connect(

    // Unsplitting 2->1
    ($output:pat <- $block:ty $params:expr {($source1:ident,$source2:ident)}) => (
        let $output = rustradio::blocks::Hack::<$block>.process(rustradio::buffers::buffer_fixed($source1.zip($source2), rustradio::DEFAULT_BUFFER_SIZE), $params);
    );

    // Splitting 1->2
    (($output1:pat,$output2:pat) <- $block:ty $params:expr {$source:ident}) => (
        let ($output1, $output2) = rustradio::buffers::split_fixed(rustradio::blocks::Hack::<$block>.process($source, ()), rustradio::DEFAULT_BUFFER_SIZE, rustradio::DEFAULT_BUFFER_SIZE);
    );

    // 1->1 block
    ($output:pat <- $block:ty $params:expr {$source:ident}) => (
        let $output = rustradio::blocks::Hack::<$block>.process(rustradio::buffers::buffer_fixed($source, rustradio::DEFAULT_BUFFER_SIZE), $params);
    );

)
