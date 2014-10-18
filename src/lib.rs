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

pub static DEFAULT_BUFFER_SIZE: uint = 2048;

/// Simplifies connecting blocks with fixed buffers.
///
/// Right now, only blocks that operate on 1 or 2 streams is supported. More
/// input/output streams should be easy to add.
///
/// TODO once Associated Items fully lands, we should be able to make this a function
/// by having blocks have static variables for number of inputs and number of outputs
#[macro_export]
macro_rules! connect(

    // Unsplitting 2->1
    ($output:pat <- $block:ident ($source1:ident, $source2:ident)) => (
        let $output = $block.process(rustradio::buffers::buffer_fixed($source1.zip($source2), rustradio::DEFAULT_BUFFER_SIZE));
    );

    // Splitting 1->2
    (($output1:pat,$output2:pat) <- $block:ident ($source:ident)) => (
        let ($output1, $output2) = rustradio::buffers::split_fixed($block.process($source), rustradio::DEFAULT_BUFFER_SIZE, rustradio::DEFAULT_BUFFER_SIZE);
    );

    // 1->1 block
    ($output:pat <- $block:ident ($source:ident)) => (
        let $output = $block.process(rustradio::buffers::buffer_fixed($source, rustradio::DEFAULT_BUFFER_SIZE));
    );

)
