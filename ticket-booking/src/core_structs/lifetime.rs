use std::fmt::Display;

// Define the Vehicle trait with a to_string method returning a &str
pub trait Vehicle {
    fn to_string(&self) -> &str;
}

// Define the Car struct
pub struct Car {
    pub name: String,
}

impl Car {
    pub fn new(name: String) -> Self {
        Car { name }
    }
}

// Implement Vehicle for Car
impl Vehicle for Car {
    fn to_string(&self) -> &str {
        &self.name
    }
}

// Process function that appends vehicle's string representation to buffer
fn process<'a, T: Vehicle>(vehicle: &'a T, buffer: &'a mut String) -> &'a str {
    buffer.push_str(&vehicle.to_string());
    buffer.as_str()
    // Here vehicle can be of type impleeming trait vevhivel or else null
}

fn main() {
    // Create a String buffer (already heap-allocated)
    let mut buffer = String::new();
    // Create a Car
    let car = Car::new(String::from("Mustang"));
    let truck = Car::new(String::from("Tesla"));
    // Process the car into the buffer

    // First process call, use result immediately
    println!("Result: {}", process(&car, &mut buffer)); // Output: Result: Mustang

    // Second process call (borrow is okay now)
    let result2 = process(&truck, &mut buffer);
    println!("Result2: {}", result2); // Output: Result2: MustangTesla
    println!("Buffer: {}", buffer); // Output: Buffer: MustangTesla

    buffer.push_str("sss");
    println!("Buffer: {}", buffer); // Output: Buffer: MustangTeslasss

    // case 3: 
    let vehicle: Option<&Car> = None;
    match vehicle {
        // if its vehicle
        Some(v) => process(v, &mut buffer),
        None => {
            println!("No vehicle provided");
            "" // Return empty &str
        }
    };
}