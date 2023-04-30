use std::io::stdin;

//function to capture user name and return it
fn what_is_your_name() -> String {
    //declare a mutable string to store the name of the user
    let mut your_name = String::new();

    //capture user input in our mut variable your_name
    //read_line is borrowing the reference to the mutable variable your_name so that it can update it
    stdin()
        .read_line(&mut your_name)
        .expect("Failed to read user input");

    //return the name
    //use trim to remove the \r\n in windows and \n in unix that gets added to the user input
    //convert to lowercase to make it uniform case for matching
    your_name.trim().to_lowercase()
}

fn main() {
    //ask the user their name

    println!("Hello, what is your name ?");

    //invoke what_is_your_name
    let name = what_is_your_name();

    //print users name
    //use {:?} placeholder to print detailed contents
    println!("Hello , {:?}", name);
}
