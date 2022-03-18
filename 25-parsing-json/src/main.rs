use serde_json::Value as JsonValue;
use serde;
use serde_derive::{Serialize, Deserialize};


fn way1() {
    let json_str = r#"
        {
            "name": "Netan Mangal",
            "age": 21
        }
    "#;

    let res = serde_json::from_str(json_str);

    if res.is_ok() {
        let json: JsonValue = res.unwrap();
        println!("Name is: {}", json["name"].as_str().unwrap());
    } else {
        println!("Opps!! Error parsing json.");
    }
}

fn way2() {

    #[derive(Serialize, Deserialize)]
    struct Person {
        name: String,
        age: i8
    }

    let json_str = r#"
        {
            "name": "Netan Mangal",
            "age": 21
        }
    "#;

    let res = serde_json::from_str(json_str);
    if res.is_ok() {
        let json: Person = res.unwrap();
        println!("Name is: {}", json.name);
    } else {
        println!("Opps!! Error parsing json.");
    }
}

fn main() {
    way1();
    way2();
}
