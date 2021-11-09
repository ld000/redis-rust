use core::mem;
use crate::sds::SdsHdr8;

mod sds;

fn main() {
    let sds = sds::sds_new_len("abc", 10);

    println!("{}", mem::size_of_val(&sds));

    let a = SdsHdr8::new(5, 10, "啊aa");
    println!("{}", sds.print());
    println!("{}", mem::size_of_val(&a));
    unsafe {
        println!("{:?}", std::str::from_utf8_unchecked(std::slice::from_raw_parts(a.buf, a.len as usize)));
    }
    // println!("{:?}", a);
    // println!("{}", std::str::from_utf8( a.buf.as_slice()).unwrap())
}
