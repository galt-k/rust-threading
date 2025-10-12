
#[cfg(test)]
mod tests {
    use crate::core_structs::worker::{Worker, WorkerManager, WorkerStatus};
    use std::sync::{Arc, Mutex};

    #[test]
    fn test_worker_creation() {
        /// Create the worker manager
        let test_worker_manager = Arc::new(Mutex::new(WorkerManager::new()));
        /// Create 5 workers.
        /// Each worker requires a Arc mutex refernce of Worker manager. 
        for _ in 0..5 {
            let test_manager_clone = test_worker_manager.clone();
            Worker::new(test_manager_clone);
        }
        ///Check if len of queue is 5
        let expected = 5;
        let result = test_worker_manager.lock().unwrap().get_idle_workers();
        assert_eq!( result, expected );
    }

    #[test]
    fn test_worker_status() {
        /// Create the worker manager
        let test_worker_manager = Arc::new(Mutex::new(WorkerManager::new()));
        /// Create 5 workers.
        /// Each worker requires a Arc mutex refernce of Worker manager. 
        for _ in 0..5 {
            let test_manager_clone = test_worker_manager.clone();
            Worker::new(test_manager_clone);
        }
        // iterate through the workers in the queue and check the status. 
        for _ in 0..5 {
            let worker = test_worker_manager.lock().unwrap().get_worker();
            match worker {
                Some(worker) => {
                    let result = worker.get_status();
                    let expected = WorkerStatus::IDLE;
                    println!("Some Worker");
                    assert_eq!(result, expected);
                }
                None => {
                    println!("No Worker");
                }
            }

        }
        
    }
}
