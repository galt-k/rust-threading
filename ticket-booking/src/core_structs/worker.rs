use std::thread::{self, JoinHandle};
use uuid::Uuid;
use std::collections::VecDeque;
use std::sync::{Arc, Mutex};



pub struct WorkerManager{
    // So, this will be multi threaded becasue.
    // Scheduler will access it to get the idle workers.
    // Worker will register themselves, once they are available to work. 
    // worker will push from the back.
    // Scheduler will pop from the queue. 
    // So, I need available workers deque to be accessed by multiple threads 
    // at once, but the worker should be 
    pub available_workers: Arc<Mutex<VecDeque<Arc<Worker>>>>,
}

// add an implementation of worker manager 
// initiate an empty VecDeque 
impl WorkerManager{
    pub fn new() -> Self {
        WorkerManager {
            available_workers: Arc::new(Mutex::new(VecDeque::new())),
        }
    }


    pub fn add_worker(&mut self, worker: Arc<Worker>) {
        self.available_workers.lock().unwrap().push_back(worker);
    }

    pub fn get_worker(&mut self) -> Option<Arc<Worker>> {
        self.available_workers.lock().unwrap().pop_front()
    }

    pub fn get_idle_workers(&self) -> usize {
        self.available_workers.lock().unwrap().len()
    }
}

pub struct Worker {
    pub thread_id: String,
    pub thread_handle: JoinHandle<()>,//Thread handle type
    // I'm sure that each worker will create its own request processor.
    // There will be no shared acces required for a Request processor
    // So, no ARC required. 
    // This attribute will own the request processor
    // At the time of worker creation, this will be empty.
    // Later while the scheduler submits a request, 
    // it will submit with this. 
    pub request_processor: Option<u32>, //RequestProcessor
    // This can be accessed by multiple threads. Especially by some thread
    // from Request scheduler or
    // Some health check monitoring system So, this has be thread safe
    // I think other readers wont update the status. 
    pub status: Arc<Mutex<WorkerStatus>>,
    // also requre a handle to WorkerManager
    pub worker_manager: Arc<Mutex<WorkerManager>>
}


impl Worker{
    // Worker will get a ARC refernced pointer. 
    pub fn new(worker_manager: Arc<Mutex<WorkerManager>>) -> (){
        // Create a new thread
        // add the refernce to the worker queue in Request scheduler.   
        let thread_id = Uuid::new_v4().to_string();
        // Ok, lets clone the worker manager and pass it to creation of the worker instance. 
        let worker_manager_clone = Arc::clone(&worker_manager);
        let worker_obj = Worker {
            thread_id: thread_id,// Assignt he thread id, can put randome for now. 
            thread_handle: thread::spawn(|| {}), // Placeholder, will replace with proper logic, // Create a Thread
            request_processor: None,
            status: Arc::new(Mutex::new(WorkerStatus::IDLE)),
            worker_manager: worker_manager
        };
        let worker = Arc::new(worker_obj);
        // add an immutable shared refernce to worker manager 
        // let mut mutex_guard_worker_manager = worker_manager_clone.lock().unwrap();
        // *mutex_guard_worker_manager.avaialble_workers.push_back(&worker); 
        
        // register worker
        worker_manager_clone.lock().unwrap().add_worker(Arc::clone(&worker));

        //worker_obj
    }

    // I just want to return the WorkerStatus directly to the caller.  
    pub fn get_status(&self) -> WorkerStatus {
        let current_status = self.status.lock().unwrap();
        *current_status
    }

    // scheduler will use this. 
    // actually, request scheudler doesnt need to care
    // about the ownership of the request processor.
    // it can pass the ownership to this method scope. 
    // pub fn process_request(&mut self, request_processor: RequestProcessor) -> bool {
    //     // assign the request processor to attribute 
    //     self.request_processor = request_processor;
    //     // Run the function on the thread. 
    //     // set the status of the worker to working


    // }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum WorkerStatus {
    IDLE = 0,
    DEAD = 1,
    WORKING = 2
}