use std::io::stdin;

//creating a struct for visitors
struct Visitor {
    name: String,
    greeting: String,
}

//implement functions for a struct with impl and the struct name
impl Visitor {
    //associated function. The parameter list does not include self
    //constructor returns type Self
    //function takes parameters of type &str and stores values of type String
    //this allows us to pass string literals to the constructor and not something like new (String::from("bert"))
    fn new(name: &str, greeting: &str) -> Self {
        Self {
            name: name.to_lowercase(),
            greeting: greeting.to_string(),
        }
    }
    //membaer function as it accepts self as a parameter
    //this is automatically passed in as a reference when you call the greet_visitor() function for the Struct instance
    fn greet_visitor(&self) {
        println!("{}", self.greeting);
    }
}

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

    let visitor_list = [
        Visitor::new("bert", "Hello Bert, Enjoy your game"),
        Visitor::new("steve", "Hello Steve, Your beer is in the fridge"),
        Visitor::new("fred", "Hello Fred, Who invited you ?"),
    ];

    //iterate over the visitor_list and check if the names match the name of the user input
    //for i in 0..visitor_list.len() => Gives a warning with clippy
    //the loop variable i is only used to index the array
    //we could mess up i in the loop body and leave room for out of bound errors
    //rust clippy  recommends using an iterator
    // for visitor in &visitor_list

    //for visitor in &visitor_list {
    //    if visitor.name == name {
    //        allow_them_in = true;
    //    }
    //}

    //Replace above for loop with Iterator implementation
    //When working with lists of data, iterators are the first place to look at the functionality you need
    //Iterators designed around function chaining
    //Iterators include a find() function  to locate data within a collection wheter it is an Array, Vector ..

    //assign the result of the iterator function chain to variable named known_visitor
    //create an iterator with iter() that contains all the data from visitor_list
    //find() runs a closure. If closure returns true, find() returns the matching value
    let known_visitor = visitor_list.iter().find(|visitor| visitor.name == name);

    //find() returns a Rust type called Option
    //Options either contain a value or they don't
    //Options have 2 possible values: Some(x) or None

    match known_visitor {
        Some(visitor) => visitor.greet_visitor(),
        None => println!("You are not on the visitor list, please leave !!"),
    }
}
