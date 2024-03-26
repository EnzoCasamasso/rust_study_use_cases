#[derive(Debug)]
pub enum Program {
    V1,
    V2,
}

#[derive(Debug)]
pub enum IpAddress {
    V4(String),
    V8(String),
}
