use std::io::Read;

fn main() {
    let mut buffer = String::new();
    std::io::stdin().read_to_string(&mut buffer).unwrap();
    let mut input = buffer.split_ascii_whitespace();

    let a: u8 = input.next().unwrap().parse().unwrap();
    let b: u8 = input.next().unwrap().parse().unwrap();

    println!("{}", if a > b { a } else { b });
}
