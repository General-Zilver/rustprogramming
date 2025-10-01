struct Car
{
    seats: i32,
    model: String,
}

impl Car
{
    fn new (s: i32, m: String) -> Car
    {
        Car
        {
            seats: s,
            model: m,
        }
        
    }
    fn get_model(&self) -> &String
    {
        return &self.model
    }
    fn set_model(&mut self, new_model: String)
    {
        self.model = new_model
    }
}
fn main() 
{
    let mut c = Car::new(4,"Tacoma".to_string());
    println!("Number of seats: {}", c.seats);
    println!("Model of Car: {}", c.get_model());
    c.set_model("Corolla".to_string());
    println!("Model of Car: {}", c.get_model())
}
