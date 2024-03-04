//Simple largest string example
pub fn largest_string<'a>(term_one: &'a str, term_two: &'a str) -> &'a str {
    if term_one.len() > term_two.len() {
        term_one
    } else {
        term_two
    }
}

pub fn format_string<'a>(a: &'a str, b: &'a str) -> String {
    let a_string = String::from(a);
    let b_string = String::from(b);
    format!("{} {}", a_string, b_string).to_string()
}
