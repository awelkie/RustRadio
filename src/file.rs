extern crate num;

use std::io::{File, Reader, BufferedReader, BufferedWriter, Open, Write};
use std::mem;
use std::raw;

pub struct ReaderIterator<Buff: Reader, T: Copy> {
    buffer: Buff,
}

impl <Buff: Reader, T: Copy> Iterator<T> for ReaderIterator<Buff, T> {
    fn next(&mut self) -> Option<T> {
        match self.buffer.read_exact(mem::size_of::<T>()) {
            Err(_) => None,
            Ok(bytes) => unsafe {
                let ptr: &u8 = mem::transmute(bytes.as_ptr());
                Some(mem::transmute_copy(ptr))
            }
        }
    }
}

pub fn read_stream<T: Copy>(filename: &Path) -> ReaderIterator<BufferedReader<File>, T> {
    let file = File::open(filename).unwrap(); // FIXME
    let reader = BufferedReader::new(file);
    ReaderIterator {
        buffer: reader
    }
}

pub fn write_stream<'r, T, I>(filename: &Path, mut input: I)
where T: Copy, I: Iterator<T> {
    let file = File::open_mode(filename, Open, Write);
    let mut writer = BufferedWriter::new(file);
    for item in input {
        let slice: &[u8] = unsafe {
            mem::transmute(raw::Slice {
                data: &item as *const _ as *const u8,
                len: mem::size_of::<T>()
            })
        };
        if writer.write(slice).is_err() {
            break;
        }
    }
}

#[test]
fn write_then_read() {
    use std::io::TempDir;
    use self::num::complex::{Complex};

    let source = vec![Complex{re: 0f32, im: 3f32},
                      Complex{re: 1f32, im: 2f32},
                      Complex{re: 2f32, im: 1f32},
                      Complex{re: 3f32, im: 0f32}];
    let temp_dir = TempDir::new("RustRadio").unwrap();
    let mut temp_file = temp_dir.path().clone();
    temp_file.set_filename("test_file");

    write_stream(&temp_file, source.iter().map(|&x| x));
    let result: Vec<Complex<f32>> = read_stream(&temp_file).collect();
    assert_eq!(source, result);
}
