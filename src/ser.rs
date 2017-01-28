extern crate byteorder;

use std::result;
use std::io;
use std::str;
use std::char;
use std::error;
use std::fmt;
use self::byteorder::{BigEndian, WriteBytesExt};

use serde::ser;

pub struct Serializer<W> {
	pub writer: W,
}

#[derive(Debug)]
pub enum Error {
	None,
	Error,
}

impl ser::Error for Error {
	fn custom<T: Into<String>>(_msg: T) -> Self {
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

impl<W> ser::Serializer for Serializer<W>
	where W: io::Write,
{
	type Error = Error;

	type SeqState = ();

    type TupleState = ();

    type TupleStructState = ();

    type TupleVariantState = ();

    type MapState = ();

    type StructState = ();

	type StructVariantState = ();

	fn serialize_bool(&mut self, v: bool) -> Result<(), self::Error> {
		let _ = self.writer.write(&[1]);
        let mut wtr = vec![];
        wtr.write_u8(v as u8).unwrap();
        for digit in &mut wtr {
	        let _ = self.writer.write(&[*digit]);
        }
		result::Result::Ok(())
	}

	fn serialize_u8(&mut self, v: u8) -> Result<(), self::Error> {
		self.serialize_u64(v as u64)
	}

	fn serialize_i8(&mut self, v: i8) -> Result<(), self::Error> {
		self.serialize_u64(v as u64)
	}

	fn serialize_u16(&mut self, v: u16) -> Result<(), self::Error> {
		self.serialize_u64(v as u64)
	}

	fn serialize_i16(&mut self, v: i16) -> Result<(), self::Error> {
		self.serialize_u64(v as u64)
	}

	fn serialize_u32(&mut self, v: u32) -> Result<(), self::Error> {
		self.serialize_u64(v as u64)
	}

	fn serialize_i32(&mut self, v: i32) -> Result<(), self::Error> {
		self.serialize_u64(v as u64)
	}

	fn serialize_u64(&mut self, v: u64) -> Result<(), self::Error> {
		self.serialize_f64(v as f64)
	}

	fn serialize_i64(&mut self, v: i64) -> Result<(), self::Error> {
		self.serialize_u64(v as u64)
	}

	fn serialize_f32(&mut self, v: f32) -> Result<(), self::Error> {
		self.serialize_f64(v as f64)
	}

	fn serialize_f64(&mut self, v: f64) -> Result<(), self::Error> {
		let _ = self.writer.write(&[0]);
        let mut wtr = vec![];
        wtr.write_f64::<BigEndian>(v).unwrap();
        for digit in &mut wtr {
        	let _ = self.writer.write(&[*digit]);
        }
		result::Result::Ok(())
	}

	fn serialize_isize(&mut self, v: isize) -> Result<(), self::Error> {
		self.serialize_f64(v as f64)
	}

	fn serialize_usize(&mut self, v: usize) -> Result<(), self::Error> {
		self.serialize_f64(v as f64)
	}

	fn serialize_bytes(&mut self, _value: &[u8]) -> Result<(), Self::Error>{
/*    	let mut state = try!(self.serialize_seq(value));
   		for b in value {
    	    try!(self.serialize_seq_elt(&mut state, b));
    	}
    	self.serialize_seq_end(state)*/
		result::Result::Ok(())
	}

	fn serialize_unit(&mut self) -> Result<(), self::Error> {
		// TODO
		result::Result::Ok(())
	}

	fn serialize_none(&mut self) -> Result<(), self::Error> {
		self.serialize_unit()
	}

	fn serialize_some<T: ser::Serialize>(&mut self, value: T) -> Result<(), self::Error> {
		value.serialize(self)
	}

	fn serialize_unit_struct(&mut self, _name: &'static str) -> Result<(), self::Error> {
		self.serialize_unit()
	}

	fn serialize_unit_variant(&mut self, _name: &'static str, _variant_index: usize, _variant: &'static str) -> Result<(), self::Error> {
		self.serialize_unit()
	}

	fn serialize_char(&mut self, v: char) -> Result<(), self::Error> 
	{
		let _ = self.writer.write(&[2]);
		let _ = self.writer.write(&[0, 1]);
		let _ = write!(self.writer, "{:?}", v);
		result::Result::Ok(())
	}

	fn serialize_str(&mut self, v: &str) -> Result<(), self::Error> 
	{
		let _ = self.writer.write(&[2]);
		let array = [(v.len() / 256) as u8, (v.len() % 256) as u8];
		let _ = self.writer.write(&array);
		let _ = self.writer.write(v.as_bytes());
		result::Result::Ok(())
	}


	fn serialize_newtype_struct<T: ser::Serialize>(&mut self, _name: &'static str, _value: T) -> Result<(), Self::Error> {
		// TODO
		result::Result::Ok(())
	}

	fn serialize_newtype_variant<T: ser::Serialize>(&mut self, _name: &'static str, _variant_index: usize, _variant: &'static str, _value: T) -> Result<(), Self::Error> {
		// TODO
		result::Result::Ok(())
	}

	fn serialize_seq(&mut self, _len: Option<usize>) -> Result<Self::SeqState, Self::Error> {
		// TODO
		result::Result::Ok(())
	}

	fn serialize_seq_elt<T: ser::Serialize>(&mut self, _state: &mut Self::SeqState, _value: T) -> Result<(), Self::Error> {
		// TODO
		result::Result::Ok(())
	}

	fn serialize_seq_end(&mut self, _state: Self::SeqState) -> Result<(), Self::Error> {
		// TODO
		result::Result::Ok(())		
	}

	fn serialize_seq_fixed_size(&mut self, size: usize) -> Result<Self::SeqState, Self::Error> {
		self.serialize_seq(Some(size))
	}

    fn serialize_tuple(&mut self, _len: usize) -> Result<Self::TupleState, Self::Error> {
		// TODO
		result::Result::Ok(())
    }

    fn serialize_tuple_elt<T: ser::Serialize>(&mut self, _state: &mut Self::TupleState, _value: T) -> Result<(), Self::Error> {
		// TODO
		result::Result::Ok(())
    }

    fn serialize_tuple_end(&mut self, _state: Self::TupleState) -> Result<(), Self::Error> {
    	// TODO
		result::Result::Ok(())
    }

    fn serialize_tuple_struct(&mut self, _name: &'static str, _len: usize) -> Result<Self::TupleStructState, Self::Error> {
    	// TODO
		result::Result::Ok(())
    }

    fn serialize_tuple_struct_elt<T: ser::Serialize>(&mut self, _state: &mut Self::TupleStructState, _value: T) -> Result<(), Self::Error> {
    	// TODO
		result::Result::Ok(())
    }

    fn serialize_tuple_struct_end(&mut self, _state: Self::TupleStructState) -> Result<(), Self::Error>{
    	// TODO
		result::Result::Ok(())
    }

    fn serialize_tuple_variant(&mut self, _name: &'static str, _variant_index: usize, _variant: &'static str, _len: usize) -> Result<Self::TupleVariantState, Self::Error> {
    	// TODO
		result::Result::Ok(())
    }

    fn serialize_tuple_variant_elt<T: ser::Serialize>(&mut self, _state: &mut Self::TupleVariantState, _value: T) -> Result<(), Self::Error> {
    	// TODO
		result::Result::Ok(())
    }

    fn serialize_tuple_variant_end(&mut self, _state: Self::TupleVariantState) -> Result<(), Self::Error> {
    	// TODO
		result::Result::Ok(())
    }

    fn serialize_map(&mut self, _len: Option<usize>) -> Result<Self::MapState, Self::Error> {
		let _ = self.writer.write(&[3]);
		result::Result::Ok(())
    }

    fn serialize_map_key<T: ser::Serialize>(&mut self, _state: &mut Self::MapState, key: T) -> Result<(), Self::Error> {
		let ser = Vec::with_capacity(128);
		let mut serializer = Serializer {writer: ser };
		let _ = key.serialize(&mut serializer);
		let _ = self.writer.write(&serializer.writer[1..]);
		result::Result::Ok(())
    }

    fn serialize_map_value<T: ser::Serialize>(&mut self, _state: &mut Self::MapState, value: T) -> Result<(), Self::Error> {
		let _ = value.serialize(self);
		result::Result::Ok(())
    }

    fn serialize_map_end(&mut self, _state: Self::MapState,) -> Result<(), Self::Error> {
		let _ = self.writer.write(&[0, 0, 9]);
		result::Result::Ok(())
    }

	fn serialize_struct(&mut self, _name: &'static str, _len: usize) -> Result<Self::StructState, Self::Error> {
    	// TODO
		result::Result::Ok(())
    }

	fn serialize_struct_elt<V: ser::Serialize>(&mut self, _state: &mut Self::StructState, _key: &'static str, _value: V) -> Result<(), Self::Error> {
    	// TODO
		result::Result::Ok(())
    }

    fn serialize_struct_end(&mut self, _state: Self::StructState) -> Result<(), Self::Error> {
    	// TODO
		result::Result::Ok(())
    }

    fn serialize_struct_variant(&mut self, _name: &'static str, _variant_index: usize, _variant: &'static str, _len: usize) -> Result<Self::StructVariantState, Self::Error> {
    	// TODO
		result::Result::Ok(())
    }

    fn serialize_struct_variant_elt<V: ser::Serialize>(&mut self, _state: &mut Self::StructVariantState, _key: &'static str, _value: V) -> Result<(), Self::Error> {
    	// TODO
		result::Result::Ok(())
    }

    fn serialize_struct_variant_end(&mut self, _state: Self::StructVariantState) -> Result<(), Self::Error> {
    	// TODO
		result::Result::Ok(())
    }
}