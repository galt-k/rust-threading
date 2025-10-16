use std::collections::HashMap;
use uuid::Uuid;
use std::sync::{RwLock, Mutex, Arc, RwLockReadGuard};
use std::collections::VecDeque;

// CoreStructs

// BookingRequest
// Seat
// Ticket
// Artist
// Concert
// Venue
// User

// Add single ton requirement for this struct. 
pub struct Concert {
    pub name: String,
    pub date_time: String, 
    pub tickets_sold: u32, 
    // Concert has a refernce to venue and Artist
    // Adding Mutex at this level is not efficient, Only one thread or process access the venue, seating plan? 
    pub venue: Arc<Venue>,
    pub artist: Arc<Artist> 
}

impl Concert {
    pub fn new(name: String, datetime: String, venue: Arc<Venue>, artist: Arc<Artist>)-> Self {
        Concert {
            name: name,
            date_time: datetime,
            tickets_sold: 0,
            venue: venue,
            artist: artist
        }
    }
}

pub struct Artist {
    pub name: String
}

impl Artist {
    pub fn new(name: String)-> Self{
        Artist {
            name: name
        }
    }
}

/// I'm not adding any mutext to seating plan, but adding at the concert level. 
pub struct Venue {
    pub location: String, 
    pub seating_plan:  Arc<Mutex<HashMap<String, Seat>>> // A map kind of data structure 
}

impl Venue {
    pub fn new(location: String, seating_plan: Arc<Mutex<HashMap<String, Seat>>> ) -> Self {
        Venue {
            location: location,
            seating_plan: seating_plan
        }
    }
}

pub struct User {
    pub user_id: String,
    pub name: String
}

impl User {
    pub fn new(name: String) -> Self {
        User {
            user_id: Uuid::new_v4().to_string(),
            name: name
        }
    }
}

// Each booking can have multple seats
// Eacb booking will have one user
// is BookingRequest a Send and Sync ? 
// Are all the attributes a send and sync? 
pub struct BookingRequest {
    /// We dont need to update request ID, so, not adding the mutex to request ID
    pub request_id: String,
    /// Since we will not be updating the user, so i'm not adding Mutex.
    pub user: Arc<User>,
    /// Multiple threads might access this field so arc Mutex. 
    pub status: Arc<RwLock<BookingStatus>>,  
    /// HAVE A REFERNCE TO LIST OF SEATS, [A2,A3,A4], We dont need to change anything so no Mutex. 
    pub seats: String,
    // add a refernce to  the Concert
}

impl BookingRequest {
    pub fn new(user: Arc<User>, selected_seats: String) -> Self {
        BookingRequest {
            request_id: Uuid::new_v4().to_string(),// Create  uuid.
            user: user, // user refernce
            status: Arc::new(RwLock::new(BookingStatus::CREATED)),// when created, the status is set to Created
            seats: selected_seats// will be used for seat parsing. not ideal way i think.
        }
    }

    pub fn change_to_confirmed(&mut self)-> bool {
        // you have to lock and update the status 
        let mut writeguard = self.status.write().unwrap();
        *writeguard = BookingStatus::CONFIRMED;
        true
    }

    pub fn change_to_cancelled(&mut self)-> bool {
        // you have to lock and update the status 
        let mut writeguard = self.status.write().unwrap();
        *writeguard = BookingStatus::CANCELLED;
        true
    }


    // my return value should be tied to booking request struct
    // is it possible without returning the guard? 
    // instead of retuning the guard, i can also return the arc clone of the status?
    // for now i'll return the clone of status
    pub fn get_status(&self)-> RwLockReadGuard<BookingStatus> {
        self.status.read().unwrap()
    }
}

pub struct Seat {
    pub row: String,
    pub number: String,
    pub status: SeatStatus//Enum type
}

impl Seat {
    pub fn new(row: String, number: String, status: SeatStatus) -> Self {
        Seat {
            row: row,
            number: number,
            status: status
        }
    }
}

pub enum SeatStatus {
    AVAILABLE = 0,
    BOOKED = 1,
    HOLD = 2
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum BookingStatus {
    CREATED = 0,
    WAITONPAYMENT = 1, 
    EXPIRED = 2,
    CANCELLED = 3,
    CONFIRMED = 4,
    PROCESSING = 5
}

// Existing SearchCriteria
#[derive(Debug, Default)]
pub struct SearchCriteria {
    pub artist: Option<String>,
    pub venue: Option<String>,
    pub date_time: Option<String>,
}

/// Here you should have a refernce to other core classes. 
/// We require a reference to the booking requests.
/// We  
pub trait Schuedulertrait {
    fn submit_request(&mut self, booking_request: &mut BookingRequest) -> &BookingRequest; 
}

// // All the thread level code should be here 
// // queue should be private to scheduler. 
// // It should create a threadpool once the Scheduler instance is created. 
// // Each thread should always check for new booking requests in the queue. 
// // They should pop the request from the queue process it. 
// // Once done they should be eligible for availability. 
// // Once the process starts they should not be available for work. 
// pub struct RequestScheduler {
//     pub queue: Arc<Mutex<VecDeque<BookingRequest>>>,
//     pub avaialble_workers: Arc<Mutex<WorkerManager>
//     /// pub processors: Arc<Vec<dyn ProcessorTrait>> // Any types that implement ProcessorTrait
// }

// impl Schuedulertrait for RequestScheduler {
//     pub fn new()-> {

//     }

//     pub fn submit_request(&mut self, booking_request: &mut BookingRequest) -> {
//         // add the request to queue
//         &self.add(booking_request);
//         // Here we have to create a new thread and run reuest processor on each thread
//         loop {
//             // loop until the queue is empty. 
//             // create a thread
//             thread::spawn {
//                 || {
//                     // pass the booking request refernce to Request processor. 
//                 }
//             }

//         }
//     }
// }

