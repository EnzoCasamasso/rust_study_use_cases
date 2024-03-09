use lifetimes::{format_string, largest_string};
use string_manipulation::{convert_and_ordering, convert_to_numbers};
use structs::UserTest;
use traits::Test;

mod lifetimes;
mod string_manipulation;
mod structs;
mod traits;

fn main() {
    //lifetimes
    let lagerst_str: &str;
    let format_str_string: String;
    let a_string = String::from("One string");
    let b_string = String::from("Another string");
    //begin inner escope
    {
        let string_one = String::from("Hello");
        let string_two = String::from("World!");
        //We'll have a problem 'borrowed value does not live long enough'
        //using 'string_one' and 'string_two'
        lagerst_str = largest_string(&a_string, &b_string);
        //Here the value is not returning lifetime str
        format_str_string = format_string(&string_one, &string_two);
    } //the variables inside inner escope will be cleaned here
      //end inner scope
    println!("{}", lagerst_str);
    println!("Format string: {}", format_str_string);
    //end lifetimes

    //string_manipulation
    let nums = "30 50 40 20 10";
    let vec_nums = convert_to_numbers(nums);
    let ordened_nums = convert_and_ordering(nums);
    println!("vec: {:?}", vec_nums);
    println!("orddened_vec: {:?}", ordened_nums);
    //end string_manipulation

    //trait and structs
    let user = UserTest::new("Enzo", 24);
    Test::print_test(&user);
    //end trait and structs
}
