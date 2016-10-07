use libc::*;

pub type tjhandle = *mut c_void;

pub const TJFLAG_NOREALLOC: c_int = 1024;

#[derive(Clone, Copy)]
#[repr(C)]
pub enum TJCS {
  TJCS_RGB = 0,
  TJCS_YCbCr = 1,
  TJCS_GRAY = 2,
  TJCS_CMYK = 3,
  TJCS_YCCK = 4,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub enum TJPF {
  TJPF_RGB = 0,
  TJPF_BGR = 1,
  TJPF_RGBX = 2,
  TJPF_BGRX = 3,
  TJPF_XBGR = 4,
  TJPF_XRGB = 5,
  TJPF_GRAY = 6,
  TJPF_RGBA = 7,
  TJPF_BGRA = 8,
  TJPF_ABGR = 9,
  TJPF_ARGB = 10,
  TJPF_CMYK = 11,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub enum TJSAMP {
  TJSAMP_444 = 0,
  TJSAMP_422 = 1,
  TJSAMP_420 = 2,
  TJSAMP_GRAY = 3,
  TJSAMP_440 = 4,
  TJSAMP_411 = 5,
}

#[link(name = "jpegturbo", kind = "static")]
extern "C" {
  pub fn tjInitCompress() -> tjhandle;
  pub fn tjCompress2(handle: tjhandle, src_buf: *const c_uchar, width: c_int, pitch: c_int, height: c_int, pixel_format: c_int, jpeg_buf: *mut *mut c_uchar, jpeg_size: *mut c_ulong, jpeg_subsamp: c_int, jpeg_qual: c_int, flags: c_int) -> c_int;
  pub fn tjBufSize(width: c_int, height: c_int, jpeg_subsamp: c_int) -> c_ulong;
  pub fn tjInitDecompress() -> tjhandle;
  pub fn tjDecompressHeader3(handle: tjhandle, jpeg_buf: *const c_uchar, jpeg_size: c_ulong, width: *mut c_int, height: *mut c_int, jpeg_subsamp: *mut c_int, jpeg_colorspace: *mut c_int) -> c_int;
  pub fn tjDecompress2(handle: tjhandle, jpeg_buf: *const c_uchar, jpeg_size: c_ulong, dst_buf: *mut c_uchar, width: c_int, pitch: c_int, height: c_int, pixel_format: c_int, flags: c_int) -> c_int;
  pub fn tjDestroy(handle: tjhandle) -> c_int;
  pub fn tjAlloc(bytes: c_int) -> *mut c_uchar;
  pub fn tjFree(buffer: *mut c_uchar);
}
