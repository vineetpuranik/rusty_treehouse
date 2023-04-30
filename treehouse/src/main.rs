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

    //allowing only specified users to the treehouse
    //use array to store the list of users allowed.
    //array holds the list of values with 2 rules. Values must be same type and array cannot change size
    //once you decide whom to admin, you cannot change the list without recompiling the program
    let visitor_list = ["bert", "steve", "fred"];

    //bool to capture if we have to allow the visitor to our treehouse
    let mut allow_them_in = false;

    //iterate over the visitor_list and check if the names match the name of the user input
    for i in 0..visitor_list.len() {
        if visitor_list[i] == name {
            allow_them_in = true;
        }
    }

    if allow_them_in == true {
        //print users name
        //use {:?} placeholder to print detailed contents
        println!("Hello , {:?}", name);
    } else {
        println!("Sorry, you are not on the list");
    }
}
