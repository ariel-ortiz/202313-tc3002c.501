fn main() {
    problems();
    checked();
    overflowing();
    saturating();
    wrapping();
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
