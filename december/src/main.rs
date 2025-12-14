
use std::thread;
use std::time::Duration;

fn simple_spawn_join() {
    println!("Intro to Concurrency");
    let steps =  Box::new(5);
    let thread = std::thread::spawn(move ||{
        // important to notice usage of closure
        // it captures the environment, and steps
        // variable becomes available in our new thread
        for step in 1..=*steps {
            std::thread::sleep(std::time::Duration::from_secs(1));
            println!("Thread step {}",step);
        }
        "Goodbye!" // important thread could return values
    });

    println!("Spawned a thread!");

    // Very important moment to understand closure captures
    // the environment
    
    //println!("steps now unavailable {}", steps);
    std::thread::sleep(std::time::Duration::from_secs(3));
    println!("Main thread slept for 3 seconds");
    // Now we join our spawned thread with it's returned value, if we don't our function just returns
    // without waiting for spawned thread
    let result = thread.join().unwrap(); // we need to unwrap result enum,because potentially thread could panick and we end up with err

    println!("Thread returned: {:?}", result);
    
}
fn share_data_between_threads_with_arc() {
    use std::sync::Arc; // atomic reference counter(smart pointer)
    
    println!("Intro to Concurrency");
    let steps =  Arc::new(5);
    let thread = {
        let steps = steps.clone();
        std::thread::spawn(move ||{
            for step in 1..=*steps {
                std::thread::sleep(std::time::Duration::from_secs(1));
                println!("Thread step {}",step);
            }
            "Goodbye!" // important thread could return values
        })
    };

    println!("Spawned a thread!");

    // Very important moment to understand closure captures
    // the environment
    
    println!("steps now available {}", steps);
    std::thread::sleep(std::time::Duration::from_secs(3));
    println!("Main thread slept for 3 seconds");
    // Now we join our spawned thread with it's returned value, if we don't our function just returns
    // without waiting for spawned thread
    let result = thread.join().unwrap(); // we need to unwrap result enum,because potentially thread could panick and we end up with err

    println!("Thread returned: {:?}", result);
}
fn share_data_between_threads_and_mutate() {
    use std::sync::Arc; // atomic reference counter(smart pointer)
    use std::sync::Mutex; // mutex -> mutual exclusive
    // smart pointer which guarantess that only one thread with lock
    // acquired will be able to mutate the value inside
    
    println!("Intro to Concurrency");
    let steps =  Arc::new(Mutex::new(5));
    let thread = {
        let steps = steps.clone();
        std::thread::spawn(move ||{
            while *steps.lock().unwrap() > 0{
                std::thread::sleep(std::time::Duration::from_secs(1));
                println!("Thread step {}",steps.lock().unwrap());
                *steps.lock().unwrap() -=1 ;
            }
            "Goodbye!" // important thread could return values
        })
    };

    println!("Spawned a thread!");

    // Very important moment to understand closure captures
    // the environment
    
    std::thread::sleep(std::time::Duration::from_secs(3));
    println!("Main thread slept for 3 seconds");
    
    let result = thread.join().unwrap(); 
    println!("Thread returned: {:?}", result);
}
fn main() 
{
    let handle = thread::spawn(||{
        for i in 1..10{
            println!("hi number {} from the spawned thread!",i);

        }
    });
    share_data_between_threads_and_mutate();
}
