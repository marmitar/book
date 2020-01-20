#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

fn main() {
    let nums = [1, 2, 3];
    let v: Vec<_> = nums.iter()
        .map(|&x| x).map(Some).collect();

    println!("{:?}", v);
}
