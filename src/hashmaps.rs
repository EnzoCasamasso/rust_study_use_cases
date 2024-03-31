use std::collections::HashMap;

pub fn month_hash() {
    let mut months = HashMap::new();
    let vec_months = vec!["January", "Febuary", "March", "April"];

    //I can use insert method to add key and value

    for &name in &vec_months {
        *months.entry(&name).or_insert(0) += 1;
    }
}
