fn concat_strings(s1: &String, s2: &String) -> String 
{
    let mut a:String = "".to_string();
    a.push_str(s1);
    a.push_str(s2);
    a
}

fn clone_and_modify(s: &String) -> String 
{
    let mut cm = s.clone();
    cm.push_str("World!");
    cm
}

#[allow(unused_variables, unused_mut)]
fn sum(total: &mut i32, low: i32, high: i32) 
{
    // Write your code here!
    *total = 0;

    for n in low..=high
    {
        *total += n;
    }
}


fn main() 
{
    let s1 = String::from("Hello, ");
    let s2 = String::from("World!");
    let result = concat_strings(&s1, &s2);
    println!("{}", result); // Should print: "Hello, World!"

    let s = String::from("Hello, ");
    let modified = clone_and_modify(&s);
    println!("Original: {}", s); // Should print: "Original: Hello, "
    println!("Modified: {}", modified); // Should print: "Modified: Hello, World!"
    
    let mut t = 0;
    sum(&mut t, 0, 100);
    println!("total = {}", t);


}


