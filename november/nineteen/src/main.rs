
// fn capture_modify_environment() {
//     fn calculator(operation: Box<dyn Fn() -> i32 + '_>)
//     {
//         let result = operation();
//         println!("Result of operation {}", result);
//     }
//     let z = 3;
//     let y = 2;
//     let x = 5;
//     calculator(Box::new(|| x + y + z));
// }
fn track_changes() {
    let mut tracker = 0;
    let mut update = || {
        // Your implementation here
        tracker += 1;
        println!("Tracker is now: {}",tracker);
    };

    update();
    update();
}
fn process_vector_with_for_loop<F>(vec: Vec<i32>, f: F) -> Vec<i32>
where
    F: Fn(i32) -> i32,
{
    let mut result = Vec::new();
    for x in vec {
        result.push(f(x)); // Apply the closure
        for x in vec {
        // The core logic: apply the passed closure 'f' to each element 'x'
        result.push(f(x));
    }

    }
    result
}

fn main() {

    let operation = |a: i32, b:i32| 
    {
        a*b
    };
    println!("Resutl: {}", operation(10,5));


    track_changes()


}
