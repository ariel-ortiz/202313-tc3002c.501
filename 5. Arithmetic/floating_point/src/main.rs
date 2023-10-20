fn main() {
    let x: f64 = 0.1;
    let y: f64 = x + x + x;
    println!("{} + {} + {} = {}", x, x, x, y);
    println!("{} == {}: {}", 0.3, y, approx_equal(0.3, y, 0.0000001));
    zeros();
    infinities();
    nans();
}

fn approx_equal(a: f64, b: f64, epsilon: f64) -> bool {
    if a == b {  // shortcut
        true
    } else {
        (a - b).abs() < epsilon
    }
}

fn zeros() {
    let x = 0.0;
    let y = 1.0;
    let z = -1.0;
    let a = x * y;
    let b = x * z;
    println!("{} * {} = {}", x, y, a);
    println!("{} * {} = {}", x, z, b);
    println!("{} == {}: {}", a, b, a == b);
}

fn infinities() {
    let x = 0.0;
    let y = 1.0;
    let z = -1.0;
    let a = y / x;
    let b = z / x;
    println!("{} / {} = {}", y, x, a);
    println!("{} / {} = {}", z, x, b);
}

fn nans() {
    let x: f64 = 0.0;
    let y: f64 = x / x;
    println!("{} / {} = {}", x, x, y);
    println!("{} == {}: {}", y, y, y == y);
    println!("y.is_nan(): {}", y.is_nan());
    println!("{} + 1 = {}", y, y + 1.0);
}
