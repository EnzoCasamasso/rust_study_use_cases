pub fn convert_to_numbers(str_base: &str) -> Vec<i32> {
    str_base
        .split(' ')
        .map(|s| s.parse::<i32>().expect("Falied to convert to interger"))
        .collect()
}

pub fn convert_and_ordering(str_base: &str) -> Vec<i32> {
    let mut nums: Vec<i32> = str_base
        .split(' ')
        .map(|s| s.parse::<i32>().expect("Falied to convert to integer"))
        .collect();
    order_crescent(&mut nums);
    nums
}

pub fn order_crescent(vec: &mut Vec<i32>) {
    vec.sort_by(|a, b| a.cmp(b));
}
