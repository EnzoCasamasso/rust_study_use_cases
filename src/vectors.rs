use std::collections::HashMap;

pub fn find_uniq(vec: Vec<i32>) -> i32 {
    let mut counts = HashMap::new();

    for &num in &vec {
        *counts.entry(num).or_insert(0) += 1;
    }

    for (&num, &count) in &counts {
        if count == 1 {
            return num;
        }
    }
    -1
}
