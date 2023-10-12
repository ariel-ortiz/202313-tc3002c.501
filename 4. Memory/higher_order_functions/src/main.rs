fn main() {
    let v1: Vec<i32> = vec![4, 8, 15, 16, 23, 42];
    let v2: Vec<i32> = v1.iter().map(|x| (*x) * 2).collect();
    let v3: Vec<f64> = v1.iter().map(|x| (*x as f64).sqrt()).collect();
    let v4: Vec<&i32> = v1.iter().filter(|x| *x % 2 == 0).collect();
    println!("{:?}", v2);
    println!("{:?}", v3);
    println!("{:?}", v4);

    let f = |x: i32, y: i32, z: i32| x + y + z; // anonymous function
    let a = f(1, 2, 3);
    println!("{}", a);

    let g = |x: u8| {
        let r = x * x;
        r
    };
    let b = g_(g(5));
    println!("{}", b);

    println!("{:?}", v1);

    let v5: Vec<&i32> = v1.iter().rev().collect();
    println!("{:?}", v5);
}

fn g_(x: u8) -> u8 {
    let r = x.saturating_mul(x);
    r
}
