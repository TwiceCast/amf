extern crate serde;

use std::fmt;

pub use self::ser::{Serializer};

pub mod ser;
pub mod value;

#[derive(Debug)]
pub enum Value<'a> {
    Number(f64),
    Bool(bool),
    String(&'a str),
    Object(Vec<(&'a str, Value<'a>)>),
//    Movieclip,
}

impl<'a> fmt::Display for Value<'a> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			&Value::Number(nbr) => write!(f, "nbr({})", nbr),
			&Value::String(str) => write!(f, "str({})", str),
			&Value::Bool(bool) => write!(f, "bool({})", bool),
			&Value::Object(ref arr) =>{
				let mut err = write!(f, "object {{\n");
				for &(key, ref value) in arr {
					err = err.and(write!(f, "{}: {}\n", key, value));
				}
				err = err.and(write!(f, "}}\n"));
				return err
			},
		}
		
	}
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
