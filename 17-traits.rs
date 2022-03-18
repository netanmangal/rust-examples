struct Person {
    name: String,
    age: u8
}

impl ToString for Person {
    fn to_string(&self) -> String {
        return format!("person: {}, {}", self.name, self.age);
    }
}

fn main() {
    let person = Person {name: "Netan".to_string(), age: 21};
    // person.to_stringa();
    println!("{}", person.to_string());
}