extern crate libc;

use ffi::*;

use libc::*;

pub mod ffi;

#[derive(Clone, Copy)]
pub enum ChromaSubsampling {
  Subsamp444,
  Subsamp422,
}

impl ChromaSubsampling {
  pub fn to_ffi(self) -> TJSAMP {
    match self {
      ChromaSubsampling::Subsamp444 => TJSAMP::TJSAMP_444,
      ChromaSubsampling::Subsamp422 => TJSAMP::TJSAMP_422,
    }
  }
}

pub struct TurbojpegEncoder {
  handle:   tjhandle,
}

unsafe impl Send for TurbojpegEncoder {}

impl Drop for TurbojpegEncoder {
  fn drop(&mut self) {
    let res = unsafe { tjDestroy(self.handle) };
    // FIXME(20160505): handle result.
  }
}

impl TurbojpegEncoder {
  pub fn create() -> Result<TurbojpegEncoder, ()> {
    let handle = unsafe { tjInitCompress() };
    if handle.is_null() {
      return Err(());
    }
    Ok(TurbojpegEncoder{
      handle:   handle,
    })
  }

  pub fn encode_rgb8(&mut self, pixels_buf: &[u8], width: usize, height: usize, /*subsamp: ChromaSubsampling*/) -> Result<Vec<u8>, ()> {
    let buf_size = unsafe { tjBufSize(width as i32, height as i32, TJSAMP::TJSAMP_422 as c_int) };
    if buf_size <= 0 {
      return Err(());
    }
    let buf_size = buf_size as usize;
    let mut jpeg_buf = Vec::with_capacity(buf_size);
    unsafe { jpeg_buf.set_len(buf_size) };
    let mut jpeg_buf_size = jpeg_buf.len() as u64;
    {
      let mut jpeg_buf_ptr = jpeg_buf.as_mut_ptr();
      let res = unsafe { tjCompress2(
          self.handle,
          pixels_buf.as_ptr(), width as i32, (3 * width) as i32, height as i32,
          TJPF::TJPF_RGB as c_int,
          &mut jpeg_buf_ptr as *mut *mut u8, &mut jpeg_buf_size as *mut u64,
          TJSAMP::TJSAMP_422 as c_int, 100, TJFLAG_NOREALLOC,
      ) };
      if res != 0 {
        return Err(());
      }
      if jpeg_buf_ptr != jpeg_buf.as_mut_ptr() {
        panic!("turbojpeg reallocated the jpeg buffer (different pointer)");
      }
      if jpeg_buf_size as usize > buf_size {
        panic!("turbojpeg reallocated the jpeg buffer (greater size)");
      }
    }
    unsafe { jpeg_buf.set_len(jpeg_buf_size as usize) };
    Ok(jpeg_buf)
  }
}

pub struct JpegHeader {
  pub width:    usize,
  pub height:   usize,
}

pub struct TurbojpegDecoder {
  handle:   tjhandle,
}

unsafe impl Send for TurbojpegDecoder {}

impl Drop for TurbojpegDecoder {
  fn drop(&mut self) {
    let res = unsafe { tjDestroy(self.handle) };
    // FIXME(20160505): handle result.
  }
}

impl TurbojpegDecoder {
  pub fn create() -> Result<TurbojpegDecoder, ()> {
    let handle = unsafe { tjInitDecompress() };
    if handle.is_null() {
      return Err(());
    }
    Ok(TurbojpegDecoder{
      handle:   handle,
    })
  }

  pub fn decode_rgb8(&mut self, jpeg_buf: &[u8]) -> Result<(JpegHeader, Vec<u8>), ()> {
    let mut width: i32 = 0;
    let mut height: i32 = 0;
    let mut subsamp: i32 = 0;
    let mut colorspace: i32 = 0;
    let res = unsafe { tjDecompressHeader3(
        self.handle,
        jpeg_buf.as_ptr(), jpeg_buf.len() as u64,
        &mut width as *mut i32, &mut height as *mut i32,
        &mut subsamp as *mut i32, &mut colorspace as *mut i32,
    ) };
    if res != 0 {
      return Err(());
    }
    let pixels_len = 3 * width as usize * height as usize;
    let mut pixels_buf = Vec::with_capacity(pixels_len);
    unsafe { pixels_buf.set_len(pixels_len) };
    let res = unsafe { tjDecompress2(
        self.handle,
        jpeg_buf.as_ptr(), jpeg_buf.len() as u64,
        pixels_buf.as_mut_ptr(),
        width, 3 * width, height,
        TJPF::TJPF_RGB as c_int, 0,
    ) };
    if res != 0 {
      return Err(());
    }
    Ok((JpegHeader{width: width as usize, height: height as usize}, pixels_buf))
  }
}
