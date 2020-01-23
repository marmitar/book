use cons::{List, cons};


pub fn main() {
    let list: List<u64> = cons![];

    println!("{:?}", list);

    println!("{:?}!", std::mem::size_of_val(&list))
}
