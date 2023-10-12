fn main() {
    let mut s = Student::new("Pepito", 123);
    let mut t = Student::new("Juanita", 199);
    s.set_gpa(100.0);
    t.set_gpa(85.0);

    println!("s = {:?}", s);
    println!("t = {:?}", t);
    s.say_hi();
    t.say_hi();
    println!("{}", s.gpa);
    println!("{}", s.get_gpa());
    s.set_gpa(99.0);
    println!("{:.2}", s.get_gpa());

    s.f();

    println!("{:?}", s);
}

#[derive(Debug)]
struct Student {
    name: String,
    id: u32,
    gpa: f64
}

impl Student {

    fn new(name: &str, id: u32) -> Student {
        Student {
            name: name.to_string(),
            id,
            gpa: 0.0
        }
    }

    fn say_hi(&self) {
        println!("{} (id: {}) says hi!", self.name, self.id);
    }

    fn get_gpa(&self) -> f64 {
        self.gpa
    }

    fn set_gpa(&mut self, new_gpa: f64) {
        self.gpa = new_gpa;
    }

    fn f(&self) {

    }
}
