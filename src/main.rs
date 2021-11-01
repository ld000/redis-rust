use core::mem;
use crate::sds::sdshdr8;

mod sds;

fn main() {
    let sds = sds::sds_new_len("abc", 20);

    println!("{}", mem::size_of_val(&sds));

    let a = sdshdr8::new(3, 11, "啊");
    println!("{}", "啊".len());
    println!("{}", mem::size_of_val(&a));
    println!("{:?}", a.buf);
    println!("{}", std::str::from_utf8( a.buf.as_slice()).unwrap())
}
