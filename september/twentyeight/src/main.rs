use std::fs::File;
use std::io::{Write, BufReader, BufRead};

struct Book 
{
    title: String,
    author: String,
    year: u16,
}

fn save_books(books: &Vec<Book>, filename: &str) 
{
    // TODO: Implement this function
    // Hint: Use File::create() and write!() macro
    let mut file = File::create(filename).unwrap();
    for b in books 
    {
        writeln!(file, "{},{},{}", b.title, b.author, b.year).expect("Failed to create file");
    }
}

fn load_books(filename: &str) -> Vec<Book> {
    let file = File::open(filename).expect("Failed to open file");
    let reader = BufReader::new(file);

    let mut out = Vec::new();
    for line in reader.lines() {
        let line = match line { Ok(l) => l, Err(_) => continue };
        if line.trim().is_empty() { continue; }

        // simplest: split by comma and use indexes
        let parts: Vec<&str> = line.split(',').collect();
        if parts.len() != 3 { continue; }

        let title  = parts[0].trim().to_string();
        let author = parts[1].trim().to_string();
        let year_s = parts[2].trim();

        if let Ok(year) = year_s.parse::<u16>() {
            out.push(Book { title, author, year });
        }
    }

    out
}
fn main() {
    let books = 
    vec![
        Book { title: "1984".to_string(), author: "George Orwell".to_string(), year: 1949 },
        Book { title: "To Kill a Mockingbird".to_string(), author: "Harper Lee".to_string(), year: 1960 },
        Book { title: "Brave New World".to_string(), author: "Aldous Huxley".to_string(), year: 1932 },
        Book { title: "Fahrenheit 451".to_string(), author: "Ray Bradbury".to_string(), year: 1953 },
        Book { title: "The Hobbit".to_string(), author: "J. R. R. Tolkien".to_string(), year: 1937 },
        Book { title: "Pride and Prejudice".to_string(), author: "Jane Austen".to_string(), year: 1813 },
        Book { title: "The Great Gatsby".to_string(), author: "F. Scott Fitzgerald".to_string(), year: 1925 },
    ];

    save_books(&books, "books.txt");
    println!("Books saved to file.");

    let loaded_books = load_books("books.txt");
    println!("Loaded books:");
    for book in loaded_books {
        println!("{} by {}, published in {}", book.title, book.author, book.year);
    }
}