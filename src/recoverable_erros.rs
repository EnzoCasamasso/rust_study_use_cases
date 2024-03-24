pub fn get_my_name<'a>(name: String) -> Result<String, &'a str> {
    match name.as_ref() {
        "Enzo" => Ok("This is correct name".to_string()),
        _ => Err("Name is not correct"),
    }
}
