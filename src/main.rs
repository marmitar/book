use book::Heaped;

#[inline]
fn add_one(x: &mut u8) {
    *x += 1
}

pub fn main() {
    let mut x = Heaped::default();
    add_one(&mut x);

    println!("{:?}", x.inner())
}
