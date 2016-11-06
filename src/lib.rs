use std::vec::Vec;

pub enum Value {
    Number(f64),
    Bool(bool),
    String(String),
    Object(Vec<Value>),
    Movieclip,
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
