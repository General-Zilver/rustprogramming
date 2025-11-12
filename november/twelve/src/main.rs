fn enroll_students()
{
    pub trait Show_info 
    { // Trait should be public if we want to allow others to implement it
            fn summarize(&self) -> String; // no body just declaration like interface
    }
    pub struct undergrad_student
    {
        pub name: String,
        pub gpa: i32,
        pub major: String,
    }
    impl Show_info for undergrad_student { 
            fn summarize(&self) -> String { 
                format!("{} is majoring in {} with a GPA of:{}", self.name, self.major, self.gpa)
            }
        }
    pub struct grad_student
    {
        pub name: String,
        pub gpa: i32,
        pub major: String,
        pub thesis: String,
    }
    impl Show_info for grad_student { 
            fn summarize(&self) -> String { 
                format!("{} is majoring in {} with a GPA of:{}. {}", self.name, self.major, self.gpa, self.thesis)
            }
        }
    let jj = undergrad_student{
            name: String::from("John John Payes"),
            gpa: i32::from(3.79),
            major: String::from("Computer Science")
        }
    let dj = undergrad_student{
            name: String::from("Daniel Johnson"),
            gpa: i32::from(3.99),
            major: String::from("Computer Science"),
            thesis: String::from("My thesis is that a  novel deep learning architecture can significantly \nimprove the accuracy of real-time object detection in autonomous\n vehicles compared to existing methods, enabling safer and more\n efficient navigation systems.")
        }
    pub struct Enrollment
    {
        pub students: Vec<Box<dyn ShowInfo>>,
    }
    impl Enrollment 
    {
        pub fn new() -> Self 
        {
            Enrollment 
            {
                students: Vec::new(),
            }
        }
    }
    pub fn add_student<T: ShowInfo + 'static>(&mut self, student: T) 
    {
        self.students.push(Box::new(student));
    }

}   
fn main() 
{
    enroll_students()
}