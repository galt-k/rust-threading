use std::sync::{Arc, Mutex, RwLock};
use std::thread;
use std::time::Duration;

pub fn main() -> Arc<Mutex<Vec<String>>> {

    let booked_seats = Arc::new(Mutex::new(Vec::new()));
    // declaring only one seatcount
    let seat_count = Arc::new(Mutex::new(1));

    let mut handles = vec![]; 

    //declare the write lock here 
    let write_latch =  Arc::new(RwLock::new(()));

    // 
    for user_id in ["A", "B"]{
        let booked_seats = Arc::clone(&booked_seats);
        let seat_count = Arc::clone(&seat_count); 
        let write_latch = Arc::clone(&write_latch);
        let user_id = user_id.to_string(); 

        let handle = thread::spawn(move || {

            // check if the seatcount is > 0
            // add the write lock here 
            let _guard = write_latch.write().unwrap();
            
            // get the count of the seats
            let count = *seat_count.lock().unwrap();
            
            
            
            println!("current Seat count - {:?}", count);

            if count > 0 {
                thread::sleep(Duration::from_millis(100));
                // Decrement the seatcount by 1
                let mut seat_count = seat_count.lock().unwrap();
                *seat_count -= 1;
                // add a sleep time, just to make sure that the race condition happens
                // Update the booking seats with user-id                
                let mut booked_seats = booked_seats.lock().unwrap();
                booked_seats.push(user_id.clone());
                println!("Great!! you have got a seat");
            } else {
                println!("Sorry seats not available");
            }
        });
        handles.push(handle);

    }

    // wait until all the threads are done running. 
    for handle in handles {
        handle.join().unwrap();
    }

    // return the booked seats here
    booked_seats

}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn race_condition_check(){
        let result = main();
        //let v1: Vec<&str> = vec!["A"];
        let mut expected: Vec<String> = vec!["A".to_string()];
        //expected.push("B".to_string());
        print!("result is {:?}", result);
        let result_locked = result.lock().unwrap();
        // check if the booked seats have both A and B? 
        let expected_len = expected.len();
        let result_len = result_locked.len();
        //assert_eq!(*result_locked, expected);
        assert_eq!(result_len, expected_len);
        
    }
}