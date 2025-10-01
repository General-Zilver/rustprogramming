#[derive(Debug)]
struct Student
{
    name: String,
    gpa: i32,
}

fn main() 
{
    let john = Student {
        name: String::from("John"),
        gpa: 3,
    };
    println!("{:?}", john);
}
