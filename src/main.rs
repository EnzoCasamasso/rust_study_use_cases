use lifetimes::{format_string, largest_string};

mod asynchronous;
mod lifetimes;

fn main() {
    //lifetimes
    let lagerst_str: &str;
    let format_str_string: String;
    let a_string = String::from("One string");
    let b_string = String::from("Another string");
    {
        let string_one = String::from("Hello");
        let string_two = String::from("World!");
        //We'll have a problem 'borrowed value does not live long enough'
        //using 'string_one' and 'string_two'
        lagerst_str = largest_string(&a_string, &b_string);
        //Here the value is not returning lifetime str
        format_str_string = format_string(&string_one, &string_two);
    }
    println!("{}", lagerst_str);
    println!("Format string: {}", format_str_string);
    //end lifetimes
}
