extern crate libc;
extern crate num;
extern crate rustradio;

use std::ptr;
use std::iter;
use std::sync::Arc;
use std::thread::Thread;
use libc::{c_void, uint32_t, c_int};
use num::Complex;

use rustradio::buffers::{Producer, Consumer, push_buffer};

#[link(name = "rtlsdr")]
extern {
    fn rtlsdr_open(dev: & *mut c_void, index: uint32_t) -> c_int;
    fn rtlsdr_get_device_count() -> uint32_t;
    fn rtlsdr_set_center_freq(dev: *mut c_void, freq: uint32_t) -> c_int;
    fn rtlsdr_get_center_freq(dev: *mut c_void) -> uint32_t;
    fn rtlsdr_set_sample_rate(dev: *mut c_void, rate: uint32_t) -> c_int;
    fn rtlsdr_get_sample_rate(dev: *mut c_void) -> uint32_t;
    fn rtlsdr_cancel_async(dev: *mut c_void) -> c_int;
    fn rtlsdr_reset_buffer(dev: *mut c_void) -> c_int;
    fn rtlsdr_close(dev: *mut c_void) -> c_int;
	fn rtlsdr_read_async(dev: *mut c_void, cb: extern fn(*const u8, u32, Arc<Producer<Complex<f32>>>),
        producer: Arc<Producer<Complex<f32>>>, buf_num: u32, buf_len: u32) -> c_int;
}

fn i2f(i: u8) -> f32 {i as f32/127.0 - 1.0}

extern fn async_callback(buf: *const u8, len: u32, producer: Arc<Producer<Complex<f32>>>) {
    let mut complex_vec: Vec<Complex<f32>> = Vec::with_capacity(len as uint / 2);
    unsafe {
        for i in iter::range_step(0, len, 2) {
            let real = *(buf.offset(i as int));
            let imag = *(buf.offset((i + 1) as int));
            let sample = Complex{re: i2f(real), im: i2f(imag)};
            complex_vec.push(sample);
        }
    }
    if let Err(_) = producer.push_slice(complex_vec.as_slice()) {
        panic!("Underflow!");
        //TODO call stop_async
    }
}

pub struct RTLSDR {
    dev_ptr: *mut c_void,
    is_streaming: bool,
    producer: Arc<Producer<Complex<f32>>>,
    consumer: Consumer<Complex<f32>>,
}

impl RTLSDR {
    pub fn new() -> Result<RTLSDR, &'static str> {
        let dev = ptr::null_mut();
        unsafe {
            for i in range(0, rtlsdr_get_device_count()) {
                if 0 == rtlsdr_open(&dev, i) {
                    let (producer, consumer) = push_buffer(16 * 32 * 512);
                    let producer_ptr = Arc::new(producer);
                    return Ok(RTLSDR { dev_ptr: dev,
                                       is_streaming: false,
                                       producer: producer_ptr,
                                       consumer: consumer});
                }
            }
        }
        Err("No devices found")
    }

    pub fn set_freq(&mut self, freq: u32) -> Result<u32, ()> {
        unsafe {
            if 0 != rtlsdr_set_center_freq(self.dev_ptr, freq) {
                Err(())
            } else {
                Ok(rtlsdr_get_center_freq(self.dev_ptr))
            }
        }
    }

    pub fn set_sample_rate(&mut self, fs: u32) -> Result<u32, ()> {
        unsafe {
            if 0 != rtlsdr_set_sample_rate(self.dev_ptr, fs) {
                Err(())
            } else {
                Ok(rtlsdr_get_sample_rate(self.dev_ptr))
            }
        }
    }
}

impl Iterator<Complex<f32>> for RTLSDR {
    fn next(&mut self) -> Option<Complex<f32>> {
        if !self.is_streaming {
            unsafe {
                rtlsdr_reset_buffer(self.dev_ptr);
            }
            let producer = self.producer.clone();
            let ptr = self.dev_ptr.clone();
            Thread::spawn(move|| {
                unsafe {
                    rtlsdr_read_async(ptr, async_callback, producer, 0, 0);
                }
            }).detach();
            self.is_streaming = true;
        }
        self.consumer.next()
    }
}

impl Drop for RTLSDR {
    fn drop(&mut self) {
        unsafe {
            if self.is_streaming {
                rtlsdr_cancel_async(self.dev_ptr);
            }
            rtlsdr_close(self.dev_ptr);
        }
    }
}
