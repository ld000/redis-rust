use std::ptr;
use std::alloc::{alloc, Layout};

// const SDS_TYPE_5: u8 = 0;
const SDS_TYPE_8: u8 = 1;
const SDS_TYPE_16: u8 = 2;
const SDS_TYPE_32: u8 = 3;
const SDS_TYPE_64: u8 = 4;

fn alloc_buf(len: usize) -> *mut u8 {
    let ptr = unsafe {
        let layout = Layout::from_size_align_unchecked(len, std::mem::size_of::<u8>());
        alloc(layout) as *mut u8
    };
    return ptr;
}

fn ptr_copy(src: &str, buf: *mut u8, len: usize) {
    unsafe { ptr::copy_nonoverlapping(src.as_ptr(), buf, len); }
}

pub trait Hdr {
    fn print(&self) -> &str;
}

#[repr(C, packed)]
pub struct SdsHdr8 {
    pub len: u8, // 1
    alloc: u8, // 1
    flags: u8,
    pub buf: *mut u8,
}

impl SdsHdr8 {
    pub fn new(len: u8, alloc: u8, init: &str) -> SdsHdr8 {
        let buf = alloc_buf(alloc as usize);
        ptr_copy(init, buf, len as usize);
        SdsHdr8 { len, alloc, flags: SDS_TYPE_8, buf }
    }
}

impl Hdr for SdsHdr8 {
    fn print(&self) -> &str {
        unsafe { std::str::from_utf8_unchecked(std::slice::from_raw_parts(self.buf, self.len as usize)) }
    }
}

#[repr(C, packed)]
pub struct SdsHdr16 {
    len: u16, // 2
    alloc: u16, // 2
    flags: u8,
    buf: *mut u8,
}

impl SdsHdr16 {
    pub fn new(len: u16, alloc: u16, init: &str) -> SdsHdr16 {
        let buf = alloc_buf(alloc as usize);
        ptr_copy(init, buf, len as usize);
        SdsHdr16 { len, alloc, flags: SDS_TYPE_16, buf }
    }
}

impl Hdr for SdsHdr16 {
    fn print(&self) -> &str {
        unsafe { std::str::from_utf8_unchecked(std::slice::from_raw_parts(self.buf, self.len as usize)) }
    }
}

#[repr(C, packed)]
pub struct SdsHdr32 {
    len: u32, // 4
    alloc: u32, // 4
    flags: u8,
    buf: *mut u8,
}

impl SdsHdr32 {
    pub fn new(len: u32, alloc: u32, init: &str) -> SdsHdr32 {
        let buf = alloc_buf(alloc as usize);
        ptr_copy(init, buf, len as usize);
        SdsHdr32 { len, alloc, flags: SDS_TYPE_32, buf }
    }
}

impl Hdr for SdsHdr32 {
    fn print(&self) -> &str {
        unsafe { std::str::from_utf8_unchecked(std::slice::from_raw_parts(self.buf, self.len as usize)) }
    }
}

#[repr(C, packed)]
pub struct SdsHdr64 {
    len: u64, // 8
    alloc: u64, // 8
    flags: u8,
    buf: *mut u8,
}

impl SdsHdr64 {
    pub fn new(len: u64, alloc: u64, init: &str) -> SdsHdr64 {
        let buf = alloc_buf(alloc as usize);
        ptr_copy(init, buf, len as usize);
        SdsHdr64 { len, alloc, flags: SDS_TYPE_64, buf }
    }
}

impl Hdr for SdsHdr64 {
    fn print(&self) -> &str {
        unsafe { std::str::from_utf8_unchecked(std::slice::from_raw_parts(self.buf, self.len as usize)) }
    }
}

pub fn sds_new_len(init: &str, initlen: usize) -> Box<dyn Hdr> {
    let sds_type = sds_req_type(initlen);

    // println!("{}", mem::size_of::<Sdshdr8>());
    // println!("{}", mem::size_of::<Sdshdr16>());
    // println!("{}", mem::size_of::<sdshdr32>());
    // println!("{}", mem::size_of::<sdshdr64>());

    match sds_type {
        SDS_TYPE_8 => return Box::new(SdsHdr8::new(init.len() as u8, initlen as u8, init)),
        SDS_TYPE_16 => return Box::new(SdsHdr16::new(init.len() as u16, initlen as u16, init)),
        SDS_TYPE_32 => return Box::new(SdsHdr32::new(init.len() as u32, initlen as u32, init)),
        SDS_TYPE_64 => return Box::new(SdsHdr64::new(init.len() as u64, initlen as u64, init)),
        // TODO 异常
        _ => return Box::new(SdsHdr8::new(init.len() as u8, initlen as u8, init)),
    }
}

fn sds_req_type(initlen: usize) -> u8 {
    if initlen < 1<<8 {
        return SDS_TYPE_8;
    }
    if initlen < 1<<16 {
        return SDS_TYPE_16;
    }
    if is_compiled_for_64_bit() {
        return if initlen < 1 << 32 {
            SDS_TYPE_32
        } else {
            SDS_TYPE_64
        }
    }

    return SDS_TYPE_32;
}

fn is_compiled_for_64_bit() -> bool {
    cfg!(target_pointer_width = "64")
}