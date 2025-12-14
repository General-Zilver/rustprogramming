fn data_paralelism_native() {
    
    use std::sync::{Arc, Mutex};
    use std::thread;
    
    
    let data = Arc::new(Mutex::new(vec![1, 2, 3, 4, 5]));
    let len = data.lock().unwrap().len();
    let mid = len / 2;


    let left_data = Arc::clone(&data);
    let left_handle = thread::spawn(move || {
        let mut left = left_data.lock().unwrap();
        for x in &mut left[..mid] {
            *x *= 2;
        }
    });

    let right_data = Arc::clone(&data);
    let right_handle = thread::spawn(move || {
        let mut right = right_data.lock().unwrap();
        for x in &mut right[mid..] {
            *x *= 2;
        }
    });

    left_handle.join().unwrap();
    right_handle.join().unwrap();

    println!("{:?}", *data.lock().unwrap()); // [2, 4, 6, 8, 10]
    
    
    // the main problem, I want you to notice, even though it seems like
    // a fine data paralelism, it has a huge problem.
    // because you are locking data vector at the moment of updating the value
    // it basically means, only one thread at a time can update value
    // even though logically it seems correct it's a overhead
}

fn main() 
{
    data_paralelism_native()
}
