extern crate byteorder;

use std::char;
use std::error;
use std::fmt;
use std::io;
use std::str;
use std::result;
use self::byteorder::{BigEndian, WriteBytesExt};

use serde::ser;

pub struct Serializer<'a, W : 'a> {
	pub writer: &'a mut W,
}

impl<'a, W : io::Write> Serializer<'a, W> {
	fn write_str(self, v: &str) -> () {
		let array = [(v.len() / 256) as u8, (v.len() % 256) as u8];
		let _ = self.writer.write(&array);
		let _ = self.writer.write(v.as_bytes());
	}

	pub fn new(writer: &'a mut W) -> Self {
		Serializer{writer: writer}
	}
}

pub struct SerializeSeq {

}

impl ser::SerializeSeq for SerializeSeq {

	type Ok = ();

	type Error = Error;

	fn serialize_element<T: ?Sized + ser::Serialize>(&mut self, _v : &T) -> result::Result<(), <Self as ser::SerializeSeq>::Error> {
		Ok(())
	}

	fn end(self) -> result::Result<<Self as ser::SerializeSeq>::Ok, <Self as ser::SerializeSeq>::Error> {
		Ok(())
	}
}

pub struct SerializeTuple {
}

impl ser::SerializeTuple for SerializeTuple {

	type Ok = ();

	type Error = Error;

	fn serialize_element<T: ?Sized + ser::Serialize>(&mut self, _value : &T) -> result::Result<(), <Self as ser::SerializeTuple>::Error> {
		Ok(())
	}

	fn end(self) -> result::Result<<Self as ser::SerializeTuple>::Ok, <Self as ser::SerializeTuple>::Error> {
		Ok(())
	}
}

pub struct SerializeTupleStruct {

}

impl ser::SerializeTupleStruct for SerializeTupleStruct {

	type Ok = ();

	type Error = Error;

	fn serialize_field<T: ?Sized + ser::Serialize>(&mut self, _v : &T) -> result::Result<(), <Self as ser::SerializeTupleStruct>::Error> {
		Ok(())
	}

	fn end(self) -> result::Result<<Self as ser::SerializeTupleStruct>::Ok, <Self as ser::SerializeTupleStruct>::Error> {
		Ok(())
	}
}

pub struct SerializeTupleVariant {

}

impl ser::SerializeTupleVariant for SerializeTupleVariant {

	type Ok = ();

	type Error = Error;

	fn serialize_field<T: ?Sized + ser::Serialize>(&mut self, _v : &T) -> result::Result<(), <Self as ser::SerializeTupleVariant>::Error> {
		Ok(())
	}

	fn end(self) -> result::Result<<Self as ser::SerializeTupleVariant>::Ok, <Self as ser::SerializeTupleVariant>::Error> {
		Ok(())
	}
}

pub struct SerializeMap<'a, W: 'a> {
	ser: Serializer<'a, W>
}

impl<'a, W : io::Write> ser::SerializeMap for SerializeMap<'a, W> {

	type Ok = ();

	type Error = Error;

	fn serialize_key<T: ?Sized + ser::Serialize>(&mut self, key : &T) -> result::Result<(), <Self as ser::SerializeMap>::Error> {
		let mut ser = Vec::with_capacity(128);
		{
			let serializer = Serializer {writer: &mut ser };
			let _ = key.serialize(serializer);
		}
		let _ = self.ser.writer.write(&ser[1..]); // While writing an object you don't put the marker for the string
		result::Result::Ok(())
	}

	fn serialize_value<T: ?Sized + ser::Serialize>(&mut self, value : &T) -> result::Result<(), <Self as ser::SerializeMap>::Error> {
		let _ = value.serialize(Serializer{writer : self.ser.writer});
		result::Result::Ok(())
	}

	fn end(self) -> result::Result<<Self as ser::SerializeMap>::Ok, <Self as ser::SerializeMap>::Error> {
		let _ = self.ser.writer.write(&[0, 0, 9]);
		result::Result::Ok(())
	}
}

pub struct SerializeStruct {

}

impl ser::SerializeStruct for SerializeStruct {
	type Ok = ();

	type Error = Error;

	fn serialize_field<T: ?Sized + ser::Serialize>(&mut self, _key: &'static str, _v : &T) -> result::Result<(), <Self as ser::SerializeStruct>::Error> {
		Ok(())
	}

	fn end(self) -> result::Result<Self::Ok, Self::Error> {
		Ok(())
	}

}

pub struct SerializeStructVariant {

}

impl ser::SerializeStructVariant for SerializeStructVariant {

	type Ok = ();

	type Error = Error;

	fn serialize_field<T: ?Sized + ser::Serialize>(&mut self, _key: &'static str, _v : &T) -> result::Result<(), <Self as ser::SerializeStructVariant>::Error> {
		Ok(())
	}

	fn end(self) -> result::Result<Self::Ok, Self::Error> {
		Ok(())
	}
}

#[derive(Debug)]
pub enum Error {
	None,
	Error,
}

impl ser::Error for Error {
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

impl<'a, W> ser::Serializer for Serializer<'a, W>
where W: io::Write,
{
	type Ok = ();

	type Error = Error;

	type SerializeSeq = SerializeSeq;

	type SerializeTuple = SerializeTuple;

	type SerializeTupleStruct = SerializeTupleStruct;

	type SerializeTupleVariant = SerializeTupleVariant;

	type SerializeMap = SerializeMap<'a, W>;

	type SerializeStruct = SerializeStruct;

	type SerializeStructVariant = SerializeStructVariant;

	fn serialize_bool(self, v: bool) -> Result<(), self::Error> {
		let _ = self.writer.write(&[0x01]);
		if v {
			let _ = self.writer.write(&[1]);
		}
		else {
			let _ = self.writer.write(&[0]);
		}
		result::Result::Ok(())
	}

	fn serialize_u8(self, v: u8) -> Result<(), self::Error> {
		self.serialize_u64(v as u64)
	}

	fn serialize_i8(self, v: i8) -> Result<(), self::Error> {
		self.serialize_u64(v as u64)
	}

	fn serialize_u16(self, v: u16) -> Result<(), self::Error> {
		self.serialize_u64(v as u64)
	}

	fn serialize_i16(self, v: i16) -> Result<(), self::Error> {
		self.serialize_u64(v as u64)
	}

	fn serialize_u32(self, v: u32) -> Result<(), self::Error> {
		self.serialize_u64(v as u64)
	}

	fn serialize_i32(self, v: i32) -> Result<(), self::Error> {
		self.serialize_u64(v as u64)
	}

	fn serialize_u64(self, v: u64) -> Result<(), self::Error> {
		self.serialize_f64(v as f64)
	}

	fn serialize_i64(self, v: i64) -> Result<(), self::Error> {
		self.serialize_u64(v as u64)
	}

	fn serialize_f32(self, v: f32) -> Result<(), self::Error> {
		self.serialize_f64(v as f64)
	}

	fn serialize_f64(self, v: f64) -> Result<(), self::Error> {
		let _ = self.writer.write(&[0x00]);
		let mut wtr = vec![];
		wtr.write_f64::<BigEndian>(v).unwrap();
		for digit in &mut wtr {
			let _ = self.writer.write(&[*digit]);
		}
		result::Result::Ok(())
	}

	fn serialize_bytes(self, _value: &[u8]) -> Result<(), Self::Error>{
		result::Result::Ok(())
	}

	fn serialize_unit(self) -> Result<(), self::Error> {
		let _ = self.writer.write(&[0x05]);
		result::Result::Ok(())
	}

	fn serialize_none(self) -> Result<(), self::Error> {
		let _ = self.writer.write(&[0x06]);
		result::Result::Ok(())
	}

	fn serialize_some<T: ser::Serialize + ?Sized>(self, value: &T) -> Result<(), self::Error> {
		value.serialize(self)
	}

	fn serialize_unit_struct(self, _name: &'static str) -> Result<(), self::Error> {
		self.serialize_unit()
	}

	fn serialize_unit_variant(self, _name: &'static str, _variant_index: usize, _variant: &'static str) -> Result<(), self::Error> {
		self.serialize_unit()
	}

	fn serialize_char(self, v: char) -> Result<(), self::Error> 
	{
		let _ = self.writer.write(&[0x02]);
		let _ = self.writer.write(&[0, 1]);
		let _ = write!(self.writer, "{:?}", v);
		result::Result::Ok(())
	}

	fn serialize_str(self, v: &str) -> Result<(), self::Error> 
	{
		let _ = self.writer.write(&[0x02]);
		self.write_str(v);
		result::Result::Ok(())
	}


	fn serialize_newtype_struct<T: ser::Serialize + ?Sized>(self, _name: &'static str, _value: &T) -> Result<(), Self::Error> {
		// TODO
		result::Result::Ok(())
	}

	fn serialize_newtype_variant<T: ser::Serialize + ?Sized>(self, _name: &'static str, _variant_index: usize, _variant: &'static str, _value: &T) -> Result<(), Self::Error> {
		// TODO
		result::Result::Ok(())
	}

	fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
		// TODO
		result::Result::Ok(SerializeSeq{})
	}

	fn serialize_seq_fixed_size(self, size: usize) -> Result<Self::SerializeSeq, Self::Error> {
		self.serialize_seq(Some(size))
	}

	fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple, Self::Error> {
		result::Result::Ok(SerializeTuple{})
	}

	fn serialize_tuple_struct(self, _name: &'static str, _len: usize) -> Result<Self::SerializeTupleStruct, Self::Error> {
		// TODO
		result::Result::Ok(SerializeTupleStruct{})
	}

	fn serialize_tuple_variant(self, _name: &'static str, _variant_index: usize, _variant: &'static str, _len: usize) -> Result<Self::SerializeTupleVariant, Self::Error> {
		// TODO
		result::Result::Ok(SerializeTupleVariant{})
	}

	fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
		match len {
			Some(n) => {
				let _ = self.writer.write(&[0x08]);
				let mut wtr = vec![];
				wtr.write_u32::<BigEndian>(n as u32).unwrap();
				for digit in &mut wtr {
					let _ = self.writer.write(&[*digit]);
				}
			},
			None => {
				let _ = self.writer.write(&[0x03]);
			}
		};
		result::Result::Ok(SerializeMap{ser: self})
	}

	fn serialize_struct(self, _name: &'static str, _len: usize) -> Result<Self::SerializeStruct, Self::Error> {
		result::Result::Ok(SerializeStruct{})
	}

	fn serialize_struct_variant(self, _name: &'static str, _variant_index: usize, _variant: &'static str, _len: usize) -> Result<Self::SerializeStructVariant, Self::Error> {
		// TODO
		result::Result::Ok(SerializeStructVariant{})
	}
}
