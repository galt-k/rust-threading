use std::clone;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::Mutex;
use std::sync::Arc;
pub struct ThreadPool {
    _handles: Vec<std::thread::JoinHandle<()>>,
    sender: Sender<Box<dyn Fn() + Send>>, 
}




impl ThreadPool {
    pub fn new(num_threads: u8) -> Self {
        println!("Creating a Thread Pool");
        let (sender, receiver) = channel::<Box<dyn Fn() + Send>>(); //Some function that return a tuple?
        // add the code for having the exclusive access to mutex
        // lock the receiver
        // do the work 
        // unlock the receiver for other threads. 
        let receiver = Arc::new(Mutex::new(receiver)); 
        let _handles = (0..num_threads)
            .map(|_| {
                let clone = receiver.clone();
                std::thread::spawn(move || loop {
                    
                    // check for work
                    // if we find work, do the work
                    
                    let work = clone.lock().unwrap().recv().unwrap();
                    println!("Start");
                    work();
                    println!("Finish");
                })
            })
            .collect();
        Self { _handles , sender}
    }
 
    pub fn execute<T: Fn() + Send + 'static>(&self, work: T) {
        self.sender.send(Box::new(work)).unwrap(); 
    }
}


pub struct SenderItem<T> {}

pub struct ReceiverItem<T> {}


pub fn channelItem<T>() -> (SenderItem<T>, ReceiverItem<T>) {}




#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let pool = ThreadPool::new(10);
        let foo= || std::thread::sleep(std::time::Duration::from_secs(1)); 
        pool.execute(foo.clone());
        pool.execute(foo);
        pool.execute(|| println!("Hello from thread"));
        std::thread::sleep(std::time::Duration::from_secs(1)); 
    }

    #[test]
    fn test_threading_sample() {

        let str = String::from("Hello world");

        // create a sample channel
        let (sender, receiver) = channel();
        std::thread::spawn( move || {
            sender.send(str.clone()).unwrap();
        });
        println!("{}","mkmk");
        assert_eq!(receiver.recv().unwrap(), "Hello world"); 

    }
}

