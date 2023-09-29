#[derive(Debug)]
enum CardinalPoints {
    North, South, East, West
}

#[derive(Debug)]
enum Color {
    Red,
    Green,
    Blue,
    RGB(u8, u8, u8)
}

fn main() {
    let mut direction1 = CardinalPoints::West;
    let mut direction2 = CardinalPoints::North;

    println!("{:?} {:?}", direction1, direction2);

    direction1 = CardinalPoints::South;
    direction2 = CardinalPoints::East;

    println!("{:?} {:?}", direction1, direction2);

    let color1 = Color::Blue;
    let color2 = Color::Green;
    let color3 = Color::Red;
    let color4 = Color::RGB(0, 255, 255);

    println!("{:?} {:?} {:?} {:?}", color1, color2, color3, color4);

    match color4 {
        Color::Red => println!("It's red!"),
        Color::Blue => println!("It's blue!"),
        Color::Green => println!("It's red!"),
        Color::RGB(x, y, z) => println!("x = {}, y = {}, z = {}", x, y, z),
    }

    let s = "123x";
    // let i = s.parse::<i32>().unwrap();
    let i = s.parse::<i32>().expect("Wtf!");
    // match r {
    //     Ok(i) => println!("{}", i * 2),
    //     Err(e) => println!("{:?}", e)
    // }
    // let i = r.unwrap();
    println!("i = {}", i);
}
