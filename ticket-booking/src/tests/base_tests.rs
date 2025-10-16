#[cfg(test)]
mod tests {
    use crate::core_structs::base::{BookingRequest, Concert, Artist, Venue, User, Seat, SeatStatus, BookingStatus };
    use std::sync::{Arc, Mutex};
    #[test]
    fn test_booking_request_creation() {
        // create a basic booking request
        let user_name = "John";
        let user = Arc::new(User::new(user_name.to_string()));
        let seats = "10A;10B";
        let booking_req = BookingRequest::new(user, seats.to_string());
        // check if the status is created
        let readlockguard = booking_req.get_status();
        // aseert that the status is created?
        assert_eq!(*readlockguard, BookingStatus::CREATED);
    }

    #[test]
    fn test_status_change() {
        let user_name = "John";
        let user = Arc::new(User::new(user_name.to_string()));
        let seats = "10A;10B";
        let mut booking_req = BookingRequest::new(user, seats.to_string());
        // check if the status is created
        booking_req.change_to_confirmed();
        let readlockguard = booking_req.get_status();

        // assert that the status is created?
        assert_eq!(*readlockguard, BookingStatus::CONFIRMED);
    }

}