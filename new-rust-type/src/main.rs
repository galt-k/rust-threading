
//use crate::RefMut;
mod ref_mut;
use ref_mut::RefMut;
use std::thread;
use std::time::Duration;


fn main() {
    
    let num_ = 5;
    let ref_mut = RefMut::new(num_);
    //let str_new = String::from("Civil war again");
    let test = ref_mut.get_data(); // getting the copy of the data
    let test_ref = ref_mut.get_ref(); // 
    let test_ptr = ref_mut.get_ptr();
    ref_mut.set_data(100);
    //unsafe { *test_ptr = 102 };
    println!("Hello, world!");
    println!("{}", test);
    println!("{}", *test_ref);
    unsafe { println!("{}", *test_ptr) };

    //let str_old = String::from("Civil war");
    let thread_join_handle = thread::spawn(move || {
        //thread::sleep(Duration::from_secs(2));
        println!("last1 - {}", test);
        2
    });
    thread::sleep(Duration::from_secs(2));
    //println!("last2 - {}", str_old);
    let res = thread_join_handle.join().unwrap();
    println!("{}",res);
    assert_eq!(res, 2);


    
}
