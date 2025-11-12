fn motivation_example() {
    // Problem: find the largest element
    // For integers 32
    fn largest_int(list: &[i32]) -> i32 {
        let mut largest = list[0];
        for &item in list.iter() {
            if item > largest {
                largest = item;
            }
        }
        largest
    }

    // For floats 32
    fn largest_float(list: &[f32]) -> f32 {
        let mut largest = list[0];
        for &item in list.iter() {
            if item > largest {
                largest = item;
            }
        }
        largest
    }

    // For char
    fn largest_char(list: &[char]) -> char {
        let mut largest = list[0];
        for &item in list.iter() {
            if item > largest {
                largest = item;
            }
        }
        largest
    }

    println!("{}", largest_int(&[1, 2, 5]));
    println!("{}", largest_float(&[1.5, 2.6, 5.9]));
    println!("{}", largest_char(&['A', 'B', 'C']));
}

fn main()
{
    motivation_example()
}