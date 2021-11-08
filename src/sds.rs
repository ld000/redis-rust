use std::{mem, ptr};
use std::alloc::{alloc, Layout};

pub type sds = Vec<u8>;

const SDS_TYPE_5: u8 = 0;
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

pub trait hdr {}

#[repr(C, packed)]
pub struct sdshdr8 {
    pub len: u8, // 1
    alloc: u8, // 1
    flags: u8,
    // TODO dynamic length array https://stackoverflow.com/questions/34684261/how-to-set-a-rust-array-length-dynamically
    pub buf: *mut u8,
}

impl sdshdr8 {
    pub fn new(len: u8, alloc: u8, init: &str) -> sdshdr8 {
        let mut buf = alloc_buf(alloc as usize);
        ptr_copy(init, buf, len as usize);
        sdshdr8{ len, alloc, flags: SDS_TYPE_8, buf }
    }
}

impl hdr for sdshdr8 {}

#[repr(C, packed)]
pub struct sdshdr16 {
    len: u16, // 2
    alloc: u16, // 2
    flags: u8, // 4
    buf: *mut u8,
}

impl sdshdr16 {
    pub fn new(len: u16, alloc: u16, init: &str) -> sdshdr16 {
        let mut buf = alloc_buf(alloc as usize);
        ptr_copy(init, buf, len as usize);
        sdshdr16{ len, alloc, flags: SDS_TYPE_16, buf }
    }
}

impl hdr for sdshdr16 {}

#[repr(C, packed)]
pub struct sdshdr32 {
    len: u32, // 4
    alloc: u32, // 4
    flags: u8, // 4
    buf: *mut u8,
}

impl sdshdr32 {
    pub fn new(len: u32, alloc: u32, init: &str) -> sdshdr32 {
        let mut buf = alloc_buf(alloc as usize);
        ptr_copy(init, buf, len as usize);
        sdshdr32{ len, alloc, flags: SDS_TYPE_32, buf }
    }
}

impl hdr for sdshdr32 {}

#[repr(C, packed)]
pub struct sdshdr64 {
    len: u64, // 8
    alloc: u64, // 8
    flags: u8, // 4
    buf: *mut u8,
}

impl sdshdr64 {
    pub fn new(len: u64, alloc: u64, init: &str) -> sdshdr64 {
        let mut buf = alloc_buf(alloc as usize);
        ptr_copy(init, buf, len as usize);
        sdshdr64{ len, alloc, flags: SDS_TYPE_64, buf }
    }
}

impl hdr for sdshdr64 {}

pub fn sds_new_len(init: &str, initlen: usize) -> Box<dyn hdr> {
    let sds_type = sds_req_type(initlen);

    // println!("{}", mem::size_of::<sdshdr8>());
    // println!("{}", mem::size_of::<sdshdr16>());
    // println!("{}", mem::size_of::<sdshdr32>());
    // println!("{}", mem::size_of::<sdshdr64>());

    match sds_type {
        SDS_TYPE_8 => return Box::new(sdshdr8::new(init.len() as u8, initlen as u8, init)),
        SDS_TYPE_16 => return Box::new(sdshdr16::new(init.len() as u16, initlen as u16, init)),
        SDS_TYPE_32 => return Box::new(sdshdr32::new(init.len() as u32, initlen as u32, init)),
        SDS_TYPE_64 => return Box::new(sdshdr64::new(init.len() as u64, initlen as u64, init)),
        _ => return Box::new(sdshdr8::new(init.len() as u8, initlen as u8, init)),
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