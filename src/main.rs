use lifetimes::largest_string;

mod asynchronous;
mod lifetimes;

fn main() {
    //lifetimes
    let lagerst_str: &str;
    let a_string = String::from("One string");
    let b_string = String::from("Another string");
    {
        let string_one = String::from("Hello");
        let string_two = String::from("Hello");
        //We'll have a problem 'borrowed value does not live long enough'
        //using 'string_one' and 'string_two'
        lagerst_str = largest_string(&a_string, &b_string);
    }
    println!("{}", lagerst_str);
    //end lifetimes
}
