fn add(x:i32,y:i32) -> i32
{
    //return x+y; //statements have semicolons ;
    //last line of code is treated as an expression
    x + y
}
fn sub(x:i32,y:i32) -> u8 // 0-255
{
    x-y
}
fn main() 
{
    let mut res = add(5,15);
    println!("{}", res);
    res = sub(25,15);
    println!("{}", res);
}
