use enums::{IpAddress, Program};
use hashmaps::month_hash;
use lifetimes::{format_string, largest_string};
use recoverable_erros::get_my_name;
use string_manipulation::{convert_and_ordering, convert_to_numbers};
use structs::UserTest;
use traits::Test;
use vectors::find_uniq;

mod closures;
mod enums;
mod hashmaps;
mod lifetimes;
mod recoverable_erros;
mod string_manipulation;
mod structs;
mod traits;
mod vectors;

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

    //recoverable_Errors
    let name = get_my_name(String::from("Enzo"));
    println!("{:?}", name);
    //end recoverable_erros

    //enum
    let mut version = Program::V1;
    println!("version: {:?}", version);
    version = Program::V2;
    println!("version now: {:?}", version);

    let ip_addr_v4 = IpAddress::V4(String::from("127.0.0.1"));
    let ip_addr_v8 = IpAddress::V8("192.168.0.1".to_string());
    println!("IP V4: {:?}", ip_addr_v4);
    println!("IP V8: {:?}", ip_addr_v8);
    //end enums

    //vector
    let numbers = vec![1, 1, 2, 2, 3, 3, 4, 5, 5];
    let uniq_number = find_uniq(numbers);
    println!("uniq number: {}", uniq_number);
    //end vector

    //HashMap

    //Try to iterate all of them
    let months = month_hash();
    println!("months: {:?}", months);
    //end hashmap
}
