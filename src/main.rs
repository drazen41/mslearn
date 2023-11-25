mod car_1;
mod loops;
mod errs;
mod ownership;
 
fn main() {
    //fn_car1();
    //fn_loops1();
    //fn_errs();
    fn_own();
}
#[derive(Debug)]
struct Highlight<'document>(&'document str);

#[allow(warnings)] // nema warninga
fn fn_own() {
    ownership::caller();
    let greeting = String::from("Hello");
    ownership::print_greeting(&greeting); // `print_greeting` takes a `&String` not an owned `String` so we borrow `greeting` with `&`
    ownership::print_greeting(&greeting); // Since `greeting` didn't move into `print_greeting` we can use it again

    let mut greeting = String::from("hello");
    ownership::change(&mut greeting);
    println!("{greeting}");

    let magic1 = String::from("abracadabra!");
    let magic2 = String::from("shazam!");

    let result = ownership::longest_word(&magic1, &magic2);
    println!("The longest magic word is {}", result);

    //let result;
    //{
    //    let magic3 = String::from("shazam!");
    //    result = ownership::longest_word(&magic1, &magic3);
    //}
    //println!("The longest magic word is {}", result);

    let text = String::from("The quick brown fox jumps over the lazy dog.");
    let fox = Highlight(&text[4..19]);
    let dog = Highlight(&text[35..43]);
    println!("{:?}", fox);
    println!("{:?}", dog);

    let name1 = "Joe";
    let name2 = "Chris";
    let name3 = "Anne";

    let mut names = Vec::new();

    assert_eq!("Joe", ownership::copy_and_return(&mut names, &name1));
    assert_eq!("Chris", ownership::copy_and_return(&mut names, &name2));
    assert_eq!("Anne", ownership::copy_and_return(&mut names, &name3));

    assert_eq!(
        names,
        vec!["Joe".to_string(), "Chris".to_string(), "Anne".to_string()]
    )    
}

#[allow(warnings)] // nema warninga
fn fn_car1(){
    // Car order #1: New, Manual, Hard top
    car_1::car_factory(String::from("Orange"), car_1::Transmission::Manual, true, 0);

    // Car order #2: Used, Semi-automatic, Convertible
    car_1::car_factory(String::from("Red"), car_1::Transmission::SemiAuto, false, 565);

    // Car order #3: Used, Automatic, Hard top
    car_1::car_factory(String::from("White"), car_1::Transmission::Automatic, true, 3000);

}

#[allow(warnings)] // nema warninga
fn fn_loops() {

    //loops::hash_map();
    

    // Initialize a hash map for the car orders
    // - Key: Car order number, i32
    // - Value: Car order details, Car struct
    use std::collections::HashMap;
    let mut orders: HashMap<i32, loops::Car> = HashMap::new();

    // Initialize counter variable
    let mut order = 1;
    // Declare a car as mutable "Car" struct
    let mut car: loops::Car;

    // Order 6 cars, increment "order" for each request
    // Car order #1: Used, Hard top
    car = loops::car_factory(order, 1000);
    orders.insert(order, car);
    println!("Car order {}: {:?}", order, orders.get(&order));

    // Car order #2: Used, Convertible
    order = order + 1;
    car = loops::car_factory(order, 2000);
    println!("{}: {:?}, Hard top = {}, {:?}, {}, {} miles", order, car.age.0, car.roof, car.motor, car.color, car.age.1);    
    orders.insert(order,car);
    // Car order #3: New, Hard top
    order = order + 1;
    car = loops::car_factory(order, 0);
    println!("{}: {:?}, Hard top = {}, {:?}, {}, {} miles", order, car.age.0, car.roof, car.motor, car.color, car.age.1);
    orders.insert(order, car);
    // Car order #4: New, Convertible
    order = order + 1;
    car = loops::car_factory(order, 0);
    println!("{}: {:?}, Hard top = {}, {:?}, {}, {} miles", order, car.age.0, car.roof, car.motor, car.color, car.age.1);

    // Car order #5: Used, Hard top
    order = order + 1;
    car = loops::car_factory(order, 3000);
    println!("{}: {:?}, Hard top = {}, {:?}, {}, {} miles", order, car.age.0, car.roof, car.motor, car.color, car.age.1);
    orders.insert(order, car);
    // Car order #6: Used, Hard top
    order = order + 1;
    car = loops:: car_factory(order, 4000);
    println!("{}: {:?}, Hard top = {}, {:?}, {}, {} miles", order, car.age.0, car.roof, car.motor, car.color, car.age.1);
    orders.insert(order, car);

}

#[allow(warnings)] // nema warninga
fn fn_loops1 (){
    use std::collections::HashMap;
    let mut orders: HashMap<i32, loops::Car> = HashMap::new();

    // Initialize counter variable
    let mut order = 1;
    // Declare a car as mutable "Car" struct
    let mut car: loops::Car;
    // Start with zero miles
    let mut miles = 0;
    while order <12 {
       
        car = loops::car_factory(order, miles);
        orders.insert(order, car);
        println!("Car order {}: {:?}", order, orders.get(&order));

        // Reset miles for order variety
        if miles == 2100 {
            miles = 0;
        } else {
            miles = miles + 700;
        }
        order +=1;
    }
}
#[allow(warnings)] // nema warninga
fn fn_errs(){
    errs::panics();
    errs::matching();

    let john = errs::Person {
        first: String::from("James"),
        middle: Some(String::from("Oliver")),
        last: String::from("Smith"),
    };
    assert_eq!(errs::build_full_name(&john), "James Oliver Smith");

    let alice = errs::Person {
        first: String::from("Alice"),
        middle: None,
        last: String::from("Stevens"),
    };
    assert_eq!(errs::build_full_name(&alice), "Alice Stevens");

    let bob = errs::Person {
        first: String::from("Robert"),
        middle: Some(String::from("Murdock")),
        last: String::from("Jones"),
    };
    assert_eq!(errs::build_full_name(&bob), "Robert Murdock Jones");

    println!("{:?}", errs::safe_division(9.0, 3.0));
    println!("{:?}", errs::safe_division(4.0, 0.0));
    println!("{:?}", errs::safe_division(0.0, 2.0));

    if errs::read_file_contents(std::path::PathBuf::from("src/main.rs")).is_ok() {
        println!("The program found the main file.");
    }
    if errs::read_file_contents(std::path::PathBuf::from("non-existent-file.txt")).is_err() {
        println!("The program reported an error for the file that doesn't exist.");
    }
}