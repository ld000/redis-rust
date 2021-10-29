use core::mem;

mod sds;

fn main() {
    let sds = sds::sds_new_len("abc", 10);

    println!("{}", mem::size_of_val(&sds));

}
