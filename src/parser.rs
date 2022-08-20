use nom::{
    bytes::complete::is_not,
    // see the "streaming/complete" paragraph lower for an explanation of these submodules
    character::complete::char,
    sequence::delimited,
    IResult,
};
use std::collections::HashMap;

pub fn parens(input: &str) -> IResult<&str, &str> {
    delimited(char('('), is_not(")"), char(')'))(input)
}

fn wat() -> HashMap<String, i32> {
    let mut map = HashMap::new();
    map.insert("foo".into(), 42);
    return map;
}

pub fn stuff() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{:?}", scores);

    let huh = scores.entry("Fancy".into());
    let wtf = huh.or_insert(42);
    *wtf += 1;

    println!("later {:?}", scores);
}

fn err() {
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_name and field_value are invalid at this point, try using them and
    // see what compiler error you get!
    // println!("{field_name} {field_value}");
}
