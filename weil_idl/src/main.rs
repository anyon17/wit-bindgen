use std::fs;
use wit_parser::Resolve;

fn main() {
    let mut resolve = Resolve::default();
    let contents = fs::read_to_string("erc20.wit").unwrap();
    let ids = resolve.push_str("erc20.wit", &contents).unwrap();

    let world = resolve.select_world(&ids, None).unwrap();
    println!("{:?}", world);
}
