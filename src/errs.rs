use std::fs::File;
use std::io::{Error, Read};
use std::path::PathBuf;

pub fn panics(){
    let fruits = vec!["banana", "apple", "coconut", "orange", "strawberry"];

    // pick the first item:
    let first = fruits.get(0);
    println!("{:?}", first);

    // pick the third item:
    let third = fruits.get(2);
    println!("{:?}", third);

    // pick the 99th item, which is non-existent:
    let non_existent = fruits.get(99);
    println!("{:?}", non_existent);
}
pub fn matching(){
    let fruits = vec!["banana", "apple", "coconut", "orange", "strawberry"];
    for &index in [0, 2, 99].iter() {
        match fruits.get(index) {
            Some(&"coconut") => println!("Coconuts are awesome!!!"),
            Some(fruit_name) => println!("It's a delicious {}!", fruit_name),
            None => println!("There is no fruit! :("),
        }
    }
}

pub struct Person {
    pub first: String,
    pub middle: Option<String>,
    pub last: String,
}
pub fn build_full_name(person: &Person) -> String {
    let mut full_name = String::new();
    full_name.push_str(&person.first);
    full_name.push_str(" ");

    // TODO: Implement the part of this function that handles the person's middle name.
    match &person.middle {
        Some(name) => {
             full_name.push_str(name);
             full_name.push_str(" ");
        }
        _ => {}
    }

    
    full_name.push_str(&person.last);
    println!("{full_name}");
    full_name
}

#[derive(Debug)]
pub struct DivisionByZeroError;

pub fn safe_division(dividend: f64, divisor: f64) -> Result<f64, DivisionByZeroError> {
    if divisor == 0.0 {
        Err(DivisionByZeroError)
    } else {
        Ok(dividend / divisor)
    }
}

pub fn read_file_contents(path: PathBuf) -> Result<String, Error> {
    let mut string = String::new();

    // Access a file at a specified path
    // ---------------------------------
    // TODO #1:
    // - Pass variable to `file` variable on success, or
    // - Return from function early if there's an error
    let mut file: File = match File::open(path) {
        Ok(file_handle) => file_handle, // todo!("Pass variable to `file` variable on success"),
        Err(io_error) => return Err(io_error), // todo!("Return from function early if there's an error")
    };

    // Read file contents into `String` variable with `read_to_string`
    // ---------------------------------
    // Success path is already filled in
    // TODO #2: Return from function early if there's an error
    match file.read_to_string(&mut string) {
        Ok(_) => (),
        Err(io_error) => return Err(io_error), //todo!("Return from function early if there's an error")
    };

    // TODO #3: Return `string` variable as expected by function signature
    //todo!("Return `string` variable")
    Ok(string)
}