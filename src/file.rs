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

/// Returns an iterator that reads a stream of elements from a file
///
/// This function assumes the file is back-to-back elements of type `T`.
///
/// # Example
/// ```no_run
/// use rustradio::file::read_stream;
/// // reads a stream of floats from file
/// let filename = Path::new("somefile.bin");
/// let mut stream = read_stream::<f32>(&filename);
/// for item in stream {
///     println!("got value {}", item);
/// }
/// ```
pub fn read_stream<T: Copy>(filename: &Path) -> ReaderIterator<BufferedReader<File>, T> {
    let file = File::open(filename).unwrap(); // FIXME
    let reader = BufferedReader::new(file);
    ReaderIterator {
        buffer: reader
    }
}

/// Reads the elements from an iterator and writes them to a file
///
/// This function will write all the elements in an iterator to file,
/// back-to-back, exactly as each element is represented in memory
///
/// # Example
/// ```no_run
/// use rustradio::file::write_stream;
/// use std::iter;
/// let source = iter::count(0u, 1);
/// write_stream(&Path::new("somefile.bin"), source);
///
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
    use num::complex::Complex;

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
