#[derive(PartialEq, Debug)]
// Declare Car struct to describe vehicle with four named fields
pub struct Car { color: String, motor: Transmission, roof: bool, age: (Age, u32) }

#[derive(PartialEq, Debug)]
// Declare enum for Car transmission type
pub enum Transmission { Manual, SemiAuto, Automatic }

#[derive(PartialEq, Debug)]
// Declare enum for Car age
pub enum Age { New, Used }

//////////////////////////////////////////////////

// Get the car quality by testing the value of the input argument
// - miles (u32)
// Return tuple with car age ("New" or "Used") and mileage
pub fn car_quality (miles: u32) -> (Age, u32) {

    ////todo!("Add conditional expression: If car has accumulated miles, return tuple for Used car with current mileage");
    if miles > 0 {
        return (Age::Used,miles);
    }
    ////todo!("Return tuple for New car with zero miles");
    (Age::New,0)
}

//////////////////////////////////////////////////

// Build a new "Car" using the values of four input arguments
// - color (String)
// - motor (Transmission enum)
// - roof (boolean, true if the car has a hard top roof)
// - miles (u32)
// Call the car_quality(miles) function to get the car age
// Return an instance of a "Car" struct with the arrow `->` syntax
pub fn car_factory(color: String, motor: Transmission, roof: bool, miles: u32) -> Car {

    // Show details about car order
    // - Check if order is for Used or New car, then check the roof type 
    // - Print details for New or Used car based on roof type
    ////todo!("Add conditional expression: If car is Used age, then check roof type");
    if car_quality(miles).0 == Age::Used {
        if roof {
            println!("Preparing a used car: {:?}, {}, Hard top, {} miles", motor, color, miles);
        } else {
            println!("Preparing a used car: {:?}, {}, Convertible, {} miles", motor, color, miles);
        }
    } else {
        if roof {
            println!("Building a new car: {:?}, {}, Hard top, {} miles", motor, color, miles);
        } else {
            println!("Building a new car: {:?}, {}, Convertible, {} miles", motor, color, miles);
        }
    }
        ////todo!("Add conditional expression: If roof is a hard top, print details");
            // Call the `println!` macro to show the car order details
            

    // Create a new "Car" instance as requested
    // - Bind first three fields to values of input arguments
    // - Bind "age" to tuple returned from car_quality(miles)
    Car {
        color: color,
        motor: motor,
        roof: roof,
        age: car_quality(miles)
    }
}

