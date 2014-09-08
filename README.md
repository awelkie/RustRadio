# Rust Radio

Rust Radio is a SDR and signal processing framework with an emphasis on simple processing block implementations.

## Iterator-based Processing Blocks
Unlike other SDR frameworks where blocks take and return discrete buffers of samples, Rust Radio's processing blocks operate on iterators. This allows each processing block to view the incoming samples as a continuous stream, and the actual buffering of samples is abstracted away, which leads to much shorter and simpler block implementations. Other benefits of iterator-based processing blocks are:
- Blocks can be used independently. No other part of the system is needed to use a particular block, which makes testing and code re-use easy. Feel free to run blocks on vectors, or lists, or anything that can be iterated over.
- Rust's generics allow us to process anything that our blocks will allow. This means that we can use floating point, fixed point, integer, or even arbitrary-precision numbers to represent our samples.
