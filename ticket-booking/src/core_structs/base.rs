use std::collections::HashMap;

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

pub struct Artist {
    pub name: String
}

/// I'm not adding any mutext to seating plan, but adding at the concert level. 
pub struct Venue {
    pub location: String, 
    pub seating_plan:  Arc<Mutex<HashMap<String, Seat>>> // A map kind of data structure 
}

pub struct User {
    pub user_id: u32
    pub name: String
}

// Each booking can have multple seats
// Eacb booking will have one user
// is BookingRequest a Send and Sync ? 
// Are all the attributes a send and sync? 
pub struct BookingRequest {
    /// We dont need to update request ID, so, not adding the mutex to request ID
    pub request_id: Arc<String>,
    /// Since we will not be updating the user, so i'm not adding Mutex.
    pub user: Arc<User>,
    /// Multiple threads might access this field so arc Mutex. 
    pub status: Arc<Mutex<BookingStatus>>,  
    /// HAVE A REFERNCE TO LIST OF SEATS, [A2,A3,A4], We dont need to change anything so no Mutex. 
    pub seats: Arc<Vec<String>>,
    // add a refernce to  the Concert
}

pub struct Seat {
    pub row: String,
    pub number: i32,
    pub status: SeatStatus//Enum type
}

pub Enum SeatStatus {
    AVAILABLE = 0,
    BOOKED = 1,
    HOLD = 2
}

pub Enum BookingStatus {
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
    fn submit_request(&mut self, booking_request: &mut BookingRequest) -> &BookingRequest 

}

// All the thread level code should be here 
// queue should be private to scheduler. 
// It should create a threadpool once the Scheduler instance is created. 
// Each thread should always check for new booking requests in the queue. 
// They should pop the request from the queue process it. 
// Once done they should be eligible for availability. 
// Once the process starts they should not be available for work. 
pub struct RequestScheduler {
    pub queue: Arc<Mutex<vecDeque<BookingRequest>>>,
    pub avaialble_workers: Arc<Mutex<WorkerManager>,
    //pub processors: Arc<Vec<dyn ProcessorTrait>> // Any types that implement ProcessorTrait
}

impl Schuedulertrait for RequestScheduler {
    pub fn new()-> {

    }

    pub fn submit_request(&mut self, booking_request: &mut BookingRequest) -> {
        // add the request to queue
        &self.add(booking_request);
        // Here we have to create a new thread and run reuest processor on each thread
        loop {
            // loop until the queue is empty. 
            // create a thread
            thread::spawn {
                || {
                    // pass the booking request refernce to Request processor. 
                }
            }

        }
    }
}

