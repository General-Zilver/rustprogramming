// fn using_function_as_parameter() 
// {
//    fn add(x: i32, y: i32) -> i32 {
//         x + y
//     }

//     fn calculator(x: i32, y: i32, operation: fn(i32, i32) -> i32) {
//         let result = operation(x, y);
//         println!("Result of operation: {}", result);    
//     }

//     calculator(1, 2, add);
//     calculator(1, 2, |x, y| x + y);
//     calculator(1, 2, |x, y| x - y);
//     calculator(1, 2, |x, y| x * y);

//     let mult    = |x,y| x * y;
//     let div     = |x:f32,y:f32| x / y;
//     let res = mult(1,2);
//     let res1 = div(1 as f32, 2 as f32);
//     println!("{},{}",res,res1)
// }
fn box_polymorphism() {
    use core::fmt::Debug;
    
    trait Animal: Debug {
        fn sound(&self) -> String;
    }
    
    #[derive(Debug)]

    struct Dog
    {
        name: String,
    };
    
    impl Animal for Dog {
        fn sound(&self) -> String {
            "Woof woof (Means my name is {Dog.name})".to_string()
        }
    }
    
    #[derive(Debug)]
    struct Cat
    {
        name: String,
    };
    
    impl Animal for Cat {
        fn sound(&self) -> String {
            ("Meow meow (Means: My name is {})",self .name).to_string()
        }
    }
    
    let mut zoo: Vec<Box<dyn Animal>> = Vec::new();
    
    zoo.push(Box::new(Dog{name:"Snoop".to_string()}));
    zoo.push(Box::new(Cat{name:"Bee".to_string()}));
    
    for animal in zoo {
        println!("{}",animal.sound());
    }
}
fn main() 
{
   box_polymorphism()
}
