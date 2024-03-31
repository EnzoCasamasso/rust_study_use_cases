use std::collections::HashMap;

pub fn month_hash() {
    let mut months = HashMap::new();
    let vec_months = vec!["January", "Febuary", "March", "April"];

    for &name in &vec_months {
        *months.entry(&name).or_insert(0) += 1;
    }
}
