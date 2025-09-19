use std::future::Future;
#[tokio::main] // Use tokio runtime to make main async
async fn main() {
    println!("Hello, world!");
    // Here foo1 doesnt return a usize.
    // what its saying is it turns to usize eventually. 
    // await -> Don't run the following instructions until the following variable returns to the output type. 
    let x = foo2(); //
    println!("value is - {}", x.await);
      

}

// 
async fn foo1() -> usize {
    0
}

//Returning some type that implements Future trait
fn foo2() -> impl Future<Output = usize> {
    async { 
        println!("in foo2");
        //let val = foo1().await;
        let val = 1000;
        val
    }  
}