fn main() {
    problems();
    checked();
    overflowing();
    saturating();
    wrapping();
    let n: u8 = 34;
    println!("{}! = {:?}", n, fact(n).unwrap_or(1));
}

fn problems() {
    let x = 127i8;
    let y = 0i8;
    let z = x + y; // overflow panic in debug mode
                   // wraparound in release mode
    println!("{} + {} = {}", x, y, z);
}

fn checked() {
    let x = 127i8;
    let y = 1i8;
    let z = x.checked_add(y);
    match z {
        Some(n) => println!("Result = {}", n),
        None => println!("No result")
    }
}

fn overflowing() {
    let x = 127i8;
    let y = 1i8;
    let z = x.overflowing_add(y);
    println!("{:?}", z);
}

fn saturating() {
    let x = 127i8;
    let y = 1i8;
    let z = x.saturating_add(y);
    println!("{}", z);
}

fn wrapping() {
    let x = 127i8;
    let y = 1i8;
    let z = x.wrapping_add(y);
    println!("{}", z);
}

fn fact(n: u8) -> Option<u128> {
    let mut r: u128 = 1;
    for i in 2..=n {
        match r.checked_mul(i as u128) {
            Some(x) => r = x,
            None => return None
        }
    }
    Some(r)
}
