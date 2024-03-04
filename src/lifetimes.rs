pub fn largest_string<'a>(term_one: &'a str, term_two: &'a str) -> &'a str {
    if term_one.len() > term_two.len() {
        term_one
    } else {
        term_two
    }
}
