fn main() {
    let mut v: Vec<String> = vec![
        "lunes".to_string(),
        "martes".to_string(),
        "miércoles".to_string()
    ];
    v.reserve(7);
    let s1: &str = "prueba";
    let s2: String = String::from("otra prueba");
    v.push(s1.to_string());
    v.push(s2.clone());
    println!("len = {}, v = {:?}", v.len(), v);
    println!("capacity = {}", v.capacity());
    println!("{}", s2);

    // (1)
    // for elem in v { // for elem in v.into_iter() {

    // (2)
    // for elem in &v { // for elem in (&v).into_iter() {
    //             v.iter()

    // (3)
    for elem in &mut v { // for elem in (&mut v).into_iter() {
    //          v.iter_mut()
        (*elem).push('!');
        println!("{}", elem);
    }

    println!("{:?}", v);
}
