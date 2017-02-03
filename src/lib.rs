#[macro_use]
extern crate serde;

use std::fmt;

pub use self::ser::Serializer;
pub use self::de::Deserializer;

pub use self::value::Value;

pub mod ser;
pub mod de;
pub mod value;

#[derive(Debug)]
pub enum OldValue<'a> {
    Number(f64),
    Bool(bool),
    String(&'a str),
    Object(Vec<(&'a str, OldValue<'a>)>),
//    Movieclip,
}

impl<'a> fmt::Display for OldValue<'a> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			&OldValue::Number(nbr) => write!(f, "nbr({})", nbr),
			&OldValue::String(str) => write!(f, "str({})", str),
			&OldValue::Bool(bool) => write!(f, "bool({})", bool),
			&OldValue::Object(ref arr) =>{
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
