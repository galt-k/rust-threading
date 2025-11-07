use std::sync::{Arc, Mutex, RwLock};
use crate::core_structs::base::{BookingRequest, Venue, Concert};
/// Do we need Arc Mutex for booking request? 
/// Multiple threads can be accessing Booking request.
/// Ex- Request processor can be accessing it and there can be another thread which might be accessing to know the status. 
pub trait ProcessorTrait {
    fn new(booking_request: Arc<RwLock<BookingRequest>>, concert: Arc<Concert> ) -> Self;
    fn process_request(&self) -> bool;
}

// I wanted a shared ownership of Booking request and Concert
// I dont have to put Mutex, because all the attributes of booking request are thread safe
// So, i'm dealing with a refences here. Both BookingRequest and Concert
// Shouldnt I think about adding lifetime annotations for these on Request processorstruct?
pub struct RequestProcessor {
    pub booking_request: Arc<RwLock<BookingRequest>>,  
    pub concert: Arc<Concert> 
}


impl ProcessorTrait for RequestProcessor {
    fn new(booking_request: Arc<RwLock<BookingRequest>>, concert: Arc<Concert> ) -> Self {
        // pass the booking request 
        // here I will have a mutable reference to booking request. 
        // Do i need to put mutex here even though all the attributes are made thread safe in BookingRequest?
        // I also need to have an immutable refernce to concert? 
        // I dont want request processoe to lock the whole booking request struct. 
        // I already ensure that individual attributes are thread safe. 
        RequestProcessor {
            booking_request: booking_request,
            concert: concert
        }
    }
    
    // here we would need life times??
    // case 1- I decided to not return anything. 
    // this function just updates the status of the booking request.
    // it wont return any refernce.
    // True
    // So, i'm dealing with a refernce here
    fn process_request(&self) -> bool {
        // check if all the seats are avaialble or not. 
        // TODO: Will update this later 
        let result: bool = true;  

        // if the seat_check avilability returns True then follow the next step
        match result {
            true => {
                // Change the seat status to "Booked"
                // TODO
                // Change the booking sttaus to "CONFIRMED" 
                // Here to update the status, you need to get the lock guard.
                // that when you will have the mutable refernce. 
                // let mut mutex_guard = self.booking_request.status.lock.unwrap();
                // *mutex_guard = BookingStatus.CONFIRMED;   
                let mut request = self.booking_request.write().unwrap();             
                request.change_to_confirmed()
            }
            false => { 
                //Acquire the lock and Change the booking status to Cancelled 
                //let mut mutex_guard = self.booking_request.status.lock.unwrap();
                //*mutex_guard = BookingStatus.CANCELLED; 
                let mut request = self.booking_request.write().unwrap(); 
                request.change_to_cancelled()
            }
        };
        // else return Booking request status to Cancelled
        true

    }
}
// Will keep this basic for now.
// GOAL-> ideally it should run any request and should have the handle to relvant 
// request processor
// Here I will only use Request processor struct and run the booking request. 
