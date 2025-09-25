fn concat_strings(s1: &String, s2: &String) -> String 
{
    let mut a: &mut String = s1;
    let b = s2;
    a.push_str(b);
    a.to_string()
}


fn main() 
{
    let s1 = String::from("Hello, ");
    let s2 = String::from("World!");
    let result = concat_strings(&s1, &s2);
    println!("{}", result); // Should print: "Hello, World!"

}

