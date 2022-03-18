struct Person {
    name: String,
    age: u8
}

impl Person {
    fn print_description(&self) {
        println!("Person {{ name: {}, age: {} }}", self.name, self.age);
    }

    fn is_eligible_for_vote(&self) -> bool {
        return self.age >= 18;
    }
}

fn main() {
    let person: Person = Person {name: "Netan".to_string(), age: 14};
    person.print_description();
    println!("Is {} eligible for vote?: {}", person.name, person.is_eligible_for_vote());
}