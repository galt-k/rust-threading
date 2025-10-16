#[cfg(test)]
mod tests {
    use crate::core_structs::processor::{RequestProcessor, ProcessorTrait};
    use crate::core_structs::base::{Concert, Venue, Seat, SeatStatus, Artist, User, BookingRequest, BookingStatus};
    use std::sync::{Arc, Mutex, RwLock};
    use std::collections::HashMap;

    #[test]
    fn test_request_processor_creation() {
        // create a booking request. 
        let user_name = "John";
        let user = Arc::new(User::new(user_name.to_string()));
        let seats = "A10;A11;A12";
        let booking_req = Arc::new(RwLock::new(BookingRequest::new(user, seats.to_string())));
        let booking_req_clone= Arc::clone(&booking_req);
        // create a list of seats and create a seating plan.
        let seating_plan = Arc::new(Mutex::new(HashMap::from([
            ("A10".to_string(), Seat::new("A".to_string(), "10".to_string(), SeatStatus::AVAILABLE)),
            ("A11".to_string(), Seat::new("A".to_string(), "11".to_string(), SeatStatus::AVAILABLE)),
            ("A12".to_string(), Seat::new("A".to_string(), "12".to_string(), SeatStatus::AVAILABLE)),
            ("A13".to_string(), Seat::new("A".to_string(), "13".to_string(), SeatStatus::AVAILABLE))
        ])));
        // seating plan seating_plan: Arc<Mutex<HashMap<String, Seat>>>
        // create a venue
        let location = "Dublin";
        let venue = Arc::new(Venue::new(location.to_string(), seating_plan));

        // create an Artist
        let artist_name = "Taylor";
        let artist = Arc::new(Artist::new(artist_name.to_string()));

        //Create a concert
        let concert = Arc::new(Concert::new(
            "Eras tour".to_string(),
            "oct 10, 2025".to_string(),
            venue,
            artist
        ));

        // request processor
        let request_proc = RequestProcessor::new(booking_req, concert);

        //process request
        request_proc.process_request();

        // check if the booking request status changed to confirmed?
        let readlockguard = booking_req_clone.read().unwrap();             
        let status_guard = readlockguard.get_status();

        // assert that the status is created?
        assert_eq!(*status_guard, BookingStatus::CONFIRMED);
    }
}