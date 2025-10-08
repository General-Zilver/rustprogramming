struct Student
{
    name: String,
        major: String
        }

        impl Student
        {
            fn new(n: String, m: String) -> Student
                {
                        Student
                                {
                                            name: n,
                                                        major: m,
                                                                }
                                                                    }
                                                                        fn get_major(&self) -> &String
                                                                            {
                                                                                    &self.major
                                                                                        }
                                                                                            fn set_major(&mut self, new_major: String) 
                                                                                                {
                                                                                                        self.major = new_major
                                                                                                            }
                                                                                                            }

                                                                                                            fn main() 
                                                                                                            {
                                                                                                                let mut s = Student::new("John".to_string(),"Computer Science".to_string());
                                                                                                                    println!("{}'s major was: {}", s.name, s.get_major() );
                                                                                                                        s.set_major("Cyber Security".to_string());
                                                                                                                            println!("{}'s major is now: {}", s.name, s.get_major()