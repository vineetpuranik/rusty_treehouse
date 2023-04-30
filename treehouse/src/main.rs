use std::io::stdin;

fn main() {
    //ask the user their name
    println!("Hello, what is your name ?");

    //declare a mutable string to store the name of the user
    let mut your_name = String::new();

    //read user input
    stdin()
        .read_line(&mut your_name)
        .expect("Failed to read line");

    //print users name
    println!("Hello, {}", your_name);
}
