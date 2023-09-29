fn main() {
    let x: i8 = -100;
    let y: i8 = 20;

    println!("{} ... {} ({} bits)", i8::MIN, i8::MAX, i8::BITS);

    let z: Option<i8> = x.checked_add(y);
    match z {
        Some(n) => println!("{} + {} = {}, {:?}", x, y, n, z),
        None => println!("¡Ups! La suma se desbordó.")
    }

    let z = x.checked_add(y).unwrap_or(100);
    println!("{} + {} = {}", x, y, z);

    let z = x.checked_sub(y).unwrap_or(-100);
    println!("{} - {} = {}", x, y, z);
}
