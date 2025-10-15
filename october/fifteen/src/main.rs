
#[derive(Debug)]
enum Fruit
{
    Apple(String),
    Banana(String),
    Tomato(String),
    Kiwi(String),
}
struct Inventory
{
    fruit: Vec<Fruit>,
}
impl Inventory
{
    fn available_fruits(&self)
    {
        for f in &self.fruit
        {
            print!("{:?} : ",f);
            Self::tell_me_joke(f);
        }
        
    }
    fn tell_me_joke(fruit:&Fruit)
    {
        match fruit 
        {
            Fruit::Apple(msg)    => println!("{}", msg),
            Fruit::Banana(msg)   => println!("{}", msg),
            Fruit::Tomato(msg)   => println!("{}", msg),
            Fruit::Kiwi(msg)     => println!("{}", msg),
        }
    }
}

fn main() 
{
    let a = "An Apple a day keeps the Doctor away.".to_string();
    let b = "A Banana boost energy in a peel.".to_string();
    let t = "A Tomato a day keeps the sunburn away.".to_string();
    let k = "A Kiwi a day, is not enough".to_string();
    let fruits = vec![Fruit::Banana(b), Fruit::Apple(a), Fruit::Tomato(t), Fruit::Kiwi(k)];

    let grocery_store = Inventory
    {
        fruit : fruits,
    };
    grocery_store.available_fruits();


}
