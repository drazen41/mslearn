#[allow(warnings)] // nema warninga
fn process(input: String) {}

pub fn caller() {
    let s = String::from("Hello, world!");
    process(s); // Ownership of the string in `s` moved into `process`
    //process(s); // Error! ownership already moved.

    let n = 1u32;
    process(n.to_string()); // Ownership of the number in `n` copied into `process`
    process(n.to_string()); // `n` can be used again because it wasn't moved, it was copied.
}
pub fn print_greeting(message: &String) {
    println!("Greeting: {}", message);
  }
  
pub fn change(text: &mut String) {
    text.push_str(", world");
}  
pub fn longest_word<'a>(x: &'a String, y: &'a String) -> &'a String {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
// TODO: modify only this function.
pub fn copy_and_return<'a>(vector: &'a mut Vec<String>, value: &'a str) -> &'a String {
    vector.push(String::from(value));
    vector.get(vector.len() - 1).unwrap()
}