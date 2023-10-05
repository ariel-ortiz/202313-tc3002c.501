fn main() {
    regla1();
    regla2();
    regla3();
}

fn regla1() {
    println!("Regla #1");
    let a = vec![1, 2, 3, 4];
    println!("a = {:?}", a);
    let b = a; // move
    println!("b = {:?}", b);
    // println!("{:?}", a); // a no es dueño del vector
    let c = b.clone();
    println!("b = {:?}", b);
    println!("c = {:?}", c);
}

fn regla2() {
    println!("Regla #2");
    let a: Vec<i32> = vec![1, 2, 3, 4];
    let b: &Vec<i32> = &a;
    let c: &Vec<i32> = &a;
    println!("a = {:?}", a);
    println!("b = {:?}", *b);
    println!("c = {:?}", *c);
    println!("a[0] = {}", a[0]);
    println!("b[0] = {}", (*b)[0]);
    println!("c[0] = {}", (*c)[0]);
}

fn regla3() {
    println!("Regla #3");
    let mut a = vec![1, 2, 3, 4];
    println!("a = {:?}", a);
    a[0] = 5;
    println!("a = {:?}", a);
    {
        let b = &mut a;
        b[2] = 7;
        // a[1] = 6; // Error por dos medios de modificación
        println!("b = {:?}", b);
    }
    a[1] = 6;
    println!("a = {:?}", a);
}
