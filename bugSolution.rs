fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    vec.push(3);

    let mut iter = vec.iter();

    match iter.next() {
        Some(val) => println!("First element: {}", val),
        None => println!("Vector is empty"),
    }

    match iter.next() {
        Some(val) => println!("Second element: {}", val),
        None => println!("Vector has only one element"),
    }

    match iter.next() {
        Some(val) => println!("Third element: {}", val),
        None => println!("End of iterator"),
    }
}