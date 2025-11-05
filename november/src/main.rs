enum Result<T, E> {
    Ok(T),
    Err(E),
}
#[derive(Debug)]
use std::fs::File;
use std::io;
use std::io::ErrorKind;
use std::io::Read;
fn read_username_from_file() -> Result<String, io::Error> 
{
    Ok("hello".to_string())
}
fn main() 
{
    let r = read_username_from_file();
    println!("{:?}",r)
}