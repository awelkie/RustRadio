extern crate num;

use std::io::{File, IoResult, Reader, BufferedReader, BufferedWriter, Open, Write};
use self::num::complex::{Complex};

pub struct ReaderIterator<'r, Buff: Reader + 'r, T> {
    buffer: Buff,
    f: |b: &mut Buff|: 'r -> IoResult<T>,
}

/// Iterates over any Reader object, stopping whenever any error is hit.
impl<'r, Buff: Reader + 'r, T> Iterator<T> for ReaderIterator<'r, Buff, T> {
    fn next(&mut self) -> Option<T> {
        (self.f)(&mut self.buffer).ok()
    }
}

pub fn read_interleaved_int16<'r>(filename: &Path) -> ReaderIterator<'r, BufferedReader<File>, Complex<i16>> {
    let file = File::open(filename).unwrap(); // FIXME
    let reader = BufferedReader::new(file);
    ReaderIterator {
        buffer: reader,
        // TODO How can we make the endianess depend on the current machine?
        f: |b: &mut BufferedReader<File>| match (b.read_le_i16(), b.read_le_i16()) {
            (Ok(re), Ok(im)) => Ok(Complex{ re: re, im: im }),
            (Err(err), _) => Err(err),
            (_, Err(err)) => Err(err),
        }
    }
}

//TODO how can we make this agnostic to the Complex type?
pub fn write_interleaved_int16<'r, I>(filename: &Path, mut input: I)
where I: Iterator<Complex<i16>> {
    let file = File::open_mode(filename, Open, Write);
    let mut writer = BufferedWriter::new(file);
    for Complex{re: i, im: q} in input {
        writer.write_le_i16(i)
            .and_then(|()| writer.write_le_i16(q))
            .unwrap_or_else(break);
    }
}
