use tokio::fs::File;
use tokio::io::AsyncWriteExt; // for write_all()
use tokio::time::{sleep, Duration, Instant}; 
use tokio::task;
use std::time::{SystemTime, UNIX_EPOCH};
use chrono::Local;

//Async function to write the content to a file
pub async fn write_to_file(filename: &str, content: &[u8]) -> std::io::Result<()> {
    let mut file = File::create(filename).await?;
    file.write_all(content).await?;
    let metadata = file.metadata().await?;
    println!("Metadata i {:?}", metadata);
    Ok(())
}

pub async fn number_writer() -> std::io::Result<()> {
    let t1 = Local::now();
    // Iterate over a loop of 100
    println!("Number writer started at {:?}", t1);
    let max_iter = 100;
    let mut count = 0u64;
    let start = std::time::Instant::now();

    // Busy loop ~1 minute
    for i in 2.. {
        let mut is_prime = true;
        for j in 2..=((i as f64).sqrt() as u64) {
            if i % j == 0 {
                is_prime = false;
                break;
            }
        }
        if is_prime {
            count += 1;
        }

        if start.elapsed() >= Duration::from_secs(60) {
            break;
        }
    }


    for number in 1..max_iter {
        sleep(Duration::from_millis(100)).await; // here i'm yielding 
        // Wait 10 sec
        //println!("100ms have elapsed after generating - {:?}", number); 
    }
    println!("Numbers created");
    let t2 = SystemTime::now();
    println!("Number writer ended at {:?}", t2);
    Ok(())
    // Write the number to file
}

pub async fn char_writer() -> std::io::Result<()> {
    let t1 = Local::now();
    println!("char_writer started at {:?}", t1);
    let max_iter = 100;
    let char_gen = "a";
    for number in 1..max_iter {
        sleep(Duration::from_millis(10)).await; // here i'm yielding 
        // Wait 10 sec
        //println!("100ms have elapsed after generating - {:?}", char_gen); 
    }
    println!("char created");
    let t2 = SystemTime::now();
    println!("Char writer ended at {:?}", t2);
    Ok(())
}

pub async fn emoji_writer() -> std::io::Result<()> {
    let t1 = Local::now();
    println!("emoji_writer started at {:?}", t1);
    let max_iter = 100;
    let char_gen = "*";
    for number in 1..max_iter {
        sleep(Duration::from_millis(10)).await; // here i'm yielding 
        // Wait 10 sec
        //println!("100ms have elapsed after generating - {:?}", char_gen); 
    }
    println!("emoji created");
    let t2 = SystemTime::now();
    println!("Emoji writer ended at {:?}", t2);
    Ok(())
}





#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[ignore]
    async fn it_works() {
        // Use the async file write function
        write_to_file("foo.txt", b"hello, world!").await.unwrap();
        
    }

    #[tokio::test]
    async fn single_thread() {
        let now = Instant::now();
        let _ = tokio::join!(
            number_writer(),
            char_writer(),
            emoji_writer()
        );
        let end_now = Instant::now(); 
        println!("completed both the functions"); 
        println!("Time taken- {:?}", end_now.checked_duration_since(now).unwrap()); 
    }

    #[tokio::test]
    #[ignore]
    async fn async_test_spawning() {
        let now = Instant::now();

        let task1 = task::spawn(async move{
            let start = Instant::now();
            println!("Number writer started at {:?}", start.checked_duration_since(now).unwrap());
            number_writer().await;
            let end = Instant::now();
            println!("Number writer ended at {:?}", end.checked_duration_since(now).unwrap());
        });

        let task2 = task::spawn(async move{
            let start = Instant::now();
            println!("Char writer started at {:?}", start.checked_duration_since(now).unwrap());
            char_writer().await;
            let end = Instant::now();
            println!("Char writer ended at {:?}", end.checked_duration_since(now).unwrap());
        });

        let task3 = task::spawn(async move{
            let start = Instant::now();
            println!("Emoji writer started at {:?}", start.checked_duration_since(now).unwrap());
            emoji_writer().await;
            let end = Instant::now();
            println!("Emoji writer ended at {:?}", end.checked_duration_since(now).unwrap());
        });

        let _ = tokio::join!(task1, task2, task3);

        let end_now = Instant::now();
        println!("All tasks done at {:?}", end_now.checked_duration_since(now).unwrap());
    }

    #[tokio::test]
    #[ignore]
    async fn async_test_no_spawing() {

        let now = Instant::now();

        number_writer().await;
        char_writer().await;
        emoji_writer().await;

        let end_now = Instant::now(); 
        println!("completed both the functions"); 
        println!("Time taken- {:?}", end_now.checked_duration_since(now).unwrap()); 
    }
}
