fn main() {
    let mut v = vec![4, 8, 15, 16, 23, 42];
    v = add_one(v);
    println!("{:?}", v);
    let u = v.clone();
    let mut w = add_one(u);
    mul_two(&mut w);
    // println!("{:?}", u);
    println!("{:?}", w);
}

fn add_one(mut a: Vec<i32>) -> Vec<i32> {
    for i in 0..a.len() {
        a[i] = a[i] + 1;
    }
    // println!("{:?}", a);
    a
}

fn mul_two(a: &mut Vec<i32>) {
    for i in 0..(*a).len() {
        (*a)[i] = (*a)[i] * 2;
    }
    (*a).push(108);
}
