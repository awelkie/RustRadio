#![feature(macro_rules)]
#![feature(globs)]
#[macro_escape]

pub mod blocks;
pub mod buffers;

pub static DEFAULT_BUFFER_SIZE: uint = 2048;

#[macro_export]
macro_rules! connect(

    // Unsplitting 2->1 with no parameters
    ($output:pat <- $block:ty $params:expr {($source1:ident,$source2:ident)}) => (
        let $output = rustradio::blocks::Hack::<$block>.process(rustradio::buffers::buffer_fixed($source1.zip($source2), rustradio::DEFAULT_BUFFER_SIZE), $params);
    );

    // Splitting 1->2 with no parameters
    (($output1:pat,$output2:pat) <- $block:ty $params:expr {$source:ident}) => (
        let ($output1, $output2) = rustradio::buffers::split_fixed(rustradio::blocks::Hack::<$block>.process($source, ()), rustradio::DEFAULT_BUFFER_SIZE, rustradio::DEFAULT_BUFFER_SIZE);
    );

    // 1->1 block with parameters
    ($output:pat <- $block:ty $params:expr {$source:ident}) => (
        let $output = rustradio::blocks::Hack::<$block>.process(rustradio::buffers::buffer_fixed($source, 10), $params);
    );

)