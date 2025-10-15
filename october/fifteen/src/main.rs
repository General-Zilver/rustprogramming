
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
    let a = "An apple a day keeps anyone away if you throw it hard enough.".to_string();
    let b = "Bananas never get lonely, they hang out in bunches.".to_string();
    let t = "Tomatoes tried to be fruit but couldn’t ketchup with the others.".to_string();
    let k = "Kiwis are like introverts. Fuzzy on the outside, sweet once you open up.".to_string();
    let fruits = vec![Fruit::Banana(b), Fruit::Apple(a), Fruit::Tomato(t), Fruit::Kiwi(k)];

    let grocery_store = Inventory
    {
        fruit : fruits,
    };
    grocery_store.available_fruits();


}
