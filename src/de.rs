use std::error;
use std::fmt;
use std::io;
use serde::de;

pub struct Deserializer<W> {
	pub reader: W,
}

//
// TODO make the Error generic between ser and de
//
#[derive(Debug)]
pub enum Error {
	None,
	Error,
}

impl de::Error for Error {
	fn custom<T: fmt::Display>(_msg: T) -> Self {
		Error::Error
	}
}

impl error::Error for Error {
	fn description(&self) -> &str {
		"Error"
	}

	fn cause(&self) -> Option<&error::Error> {
		None
	}
}

impl fmt::Display for Error {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "Error")
	}
}

impl<W> de::Deserializer for Deserializer<W>
	where W: io::Read,
{

	type Error = Error;

	fn deserialize<T: de::Visitor>(self, visitor : T) -> Result<T::Value, self::Error> {
		visitor.visit_unit()
	}

/*	fn deserialize_bool<T : de::Visitor>(self, _v : T) -> Result<T::Value, self::Error> {

	}

	fn deserialize_u8<T : de::Visitor>(self, _v : T) -> Result<T::Value, self::Error> {

	}

	fn deserialize_i8<T : de::Visitor>(self, _v : T) -> Result<T::Value, self::Error> {

	}

	fn deserialize_u16<T : de::Visitor>(self, _v : T) -> Result<T::Value, self::Error> {

	}

	fn deserialize_i16<T : de::Visitor>(self, _v : T) -> Result<T::Value, self::Error> {

	}

	fn deserialize_u32<T : de::Visitor>(self, _v : T) -> Result<T::Value, self::Error> {

	}

	fn deserialize_i32<T : de::Visitor>(self, _v : T) -> Result<T::Value, self::Error> {

	}

	fn deserialize_u64<T : de::Visitor>(self, _v : T) -> Result<T::Value, self::Error> {

	}

	fn deserialize_i64<T : de::Visitor>(self, _v : T) -> Result<T::Value, self::Error> {

	}

	fn deserialize_f32<T : de::Visitor>(self, _v : T) -> Result<T::Value, self::Error> {

	}

	fn deserialize_f64<T : de::Visitor>(self, _v : T) -> Result<T::Value, self::Error> {

	}

	fn deserialize_char<T : de::Visitor>(self, _v : T) -> Result<T::Value, self::Error> {

	}

	fn deserialize_str<T : de::Visitor>(self, _v : T) -> Result<T::Value, self::Error> {

	}

	fn deserialize_string<T : de::Visitor>(self, _v : T) -> Result<T::Value, self::Error> {

	}

	fn deserialize_bytes<T : de::Visitor>(self, _v : T) -> Result<T::Value, self::Error> {

	}

	fn deserialize_byte_buf<T : de::Visitor>(self, _v : T) -> Result<T::Value, self::Error> {

	}

	fn deserialize_option<T : de::Visitor>(self, _v : T) -> Result<T::Value, self::Error> {

	}

	fn deserialize_unit<T : de::Visitor>(self, _v : T) -> Result<T::Value, self::Error> {

	}

	fn deserialize_unit_struct<T : de::Visitor>(self, _name: &'static str, _v : T) -> Result<T::Value, self::Error> {

	}

	fn deserialize_seq<T : de::Visitor>(self, _v : T) -> Result<T::Value, self::Error> {

	}

	fn deserialize_seq_fixed_size<T : de::Visitor>(self, _len: usize, _v : T) -> Result<T::Value, self::Error> {

	}

	fn deserialize_newtype_struct<T : de::Visitor>(self, _name: &'static str, _v : T) -> Result<T::Value, self::Error> {

	}

	fn deserialize_tuple<T : de::Visitor>(self, _len: usize, _v : T) -> Result<T::Value, self::Error> {

	}

	fn deserialize_tuple_struct<T : de::Visitor>(self, _name: &'static str, _len: usize, _v : T) -> Result<T::Value, self::Error> {

	}
	
	fn deserialize_map<T : de::Visitor>(self, _v : T) -> Result<T::Value, self::Error> {

	}
	
	fn deserialize_struct<T : de::Visitor>(self, _name: &'static str, _s: &'static [&'static str], _v : T) -> Result<T::Value, self::Error> {

	}*/

/*	fn deserialize_struct_field<T : de::Visitor>(self, visitor : T) -> Result<T::Value, self::Error> {
		Ok(visitor.visit_struct_field())
	}*/

/*	fn deserialize_enum<T : de::Visitor>(self, _name: &'static str, _s: &'static [&'static str], visitor : T) -> Result<T::Value, self::Error> {
		visitor.visit_enum(EnumVisitor{})
	}

	fn deserialize_ignored_any<T : de::Visitor>(self, visitor : T) -> Result<T::Value, self::Error> {
		Err(self::Error())
	}*/

	forward_to_deserialize! {
        bool u8 u16 u32 u64 i8 i16 i32 i64 f32 f64 char str string
        unit option seq seq_fixed_size bytes byte_buf map unit_struct
        newtype_struct tuple_struct struct struct_field tuple enum ignored_any
	}
	
}