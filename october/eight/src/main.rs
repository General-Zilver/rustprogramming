use std::fs::File;
use std::io::{Read, BufReader, BufRead};

fn read_entire_file() {
    let mut file = File::open("example.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    println!("File contents:\n{}", contents);
}

fn read_file_line_by_line() {
    let file = File::open("example.txt").unwrap();
    let reader = BufReader::new(file);
    let mut my_strings: [Strings; 10] = [String::new(); 10];
    let mut i = 0;
    for line in reader.lines() {
        println!("{}", line.unwrap());
        my_strings[i]

        i++;
        if i == 10
        {
            i = 0;
        }

    }
    let mut newfile = File::create("Last_10_lines.txt").unwrap();
    for j in my_strings
    {
        writeln!(newfile, "{}",j).unwrap()
    }
}

fn main() {
    println!("Reading entire file:");
    read_entire_file();

    println!("\nReading file line by line:");
    read_file_line_by_line();
}