    pub trait ShowInfo 
    { 
            fn summarize(&self) -> String; 
    }
    pub struct UndergradStudent
    {
        pub name: String,
        pub gpa: f32,
        pub major: String,
    }
    impl ShowInfo for UndergradStudent 
    { 
        fn summarize(&self) -> String 
        { 
            format!("{} is majoring in {} with a GPA of:{}", self.name, self.major, self.gpa)
        }
    }
    pub struct GradStudent
    {
        pub name: String,
        pub gpa: f32,
        pub major: String,
        pub thesis: String,
    }
    impl ShowInfo for GradStudent 
    { 
        fn summarize(&self) -> String 
        { 
            format!("{} is majoring in {} with a GPA of:{}. {}", self.name, self.major, self.gpa, self.thesis)
        }
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
        pub fn add_student<T: ShowInfo + 'static>(&mut self, student: T) 
        {
            self.students.push(Box::new(student));
        }
        pub fn print_summaries(&self) 
        {
            for student in &self.students 
            {
                println!("{}", student.summarize());
            }
        } 
    }
fn main() 
{
    let jj = UndergradStudent 
    {
        name: String::from("John John Payes"),
        gpa: 3.79, // Just use the float literal
        major: String::from("Computer Science"),
    };

    let dj = GradStudent 
    {
        name: String::from("Daniel Johnson"),
        gpa: 3.99,
        major: String::from("Computer Science"),
        thesis: String::from("A novel deep learning architecture that improves real-time object detection.
	    This method enables safer and more efficient navigation systems."),
    };

    let mut enrollment = Enrollment::new();


    enrollment.add_student(jj);
    enrollment.add_student(dj);

    println!("--- Current Enrollment ---");
    enrollment.print_summaries();
    println!("--------------------------");
}