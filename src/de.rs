extern crate byteorder;

use std::{error, fmt, io};
use serde::de;
use serde;
use reader::Read;
use self::byteorder::{BigEndian, ReadBytesExt};

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

impl<W> Deserializer<W>
	where W: Read
{
	fn parse_string(&mut self) -> String {
		let mut tab = Vec::new();
		for _ in 0..2 {
			tab.push(self.reader.next().unwrap().unwrap());
		}
		let mut c = io::Cursor::new(tab);
		let nb = c.read_u16::<BigEndian>().unwrap();
		let mut str = Vec::new();
		for _ in 0..nb {
			str.push(self.reader.next().unwrap().unwrap());
		}
		String::from_utf8(str).unwrap()
	}

	fn parse_string_or_end(&mut self) -> Option<String> {
		let mut tab = Vec::new();
		for _ in 0..2 {
			tab.push(self.reader.next().unwrap().unwrap());
		}
		let mut c = io::Cursor::new(tab);
		let nb = c.read_u16::<BigEndian>().unwrap();
		if nb == 0
		{
			match self.reader.next().unwrap() {
				Some(0x09) => None,
				_ => None //Error
			}
		}
		else {
			let mut str = Vec::new();
			for _ in 0..nb {
				str.push(self.reader.next().unwrap().unwrap());
			}
			Some(String::from_utf8(str).unwrap())
		}
	}

	fn parse_value<T: de::Visitor>(&mut self, visitor: T) -> Result<T::Value, self::Error> {
		let c = self.reader.next().unwrap();
		match c {
			Some(0x00) => {
				let mut tab = Vec::new();
				for _ in 0..8 {
					tab.push(self.reader.next().unwrap().unwrap());
				}
				let mut c = io::Cursor::new(tab);
				let nb = c.read_f64::<BigEndian>().unwrap();
				visitor.visit_f64(nb)
			},
			Some(0x01) => {
				let tab = vec![self.reader.next().unwrap().unwrap()];
				let mut c = io::Cursor::new(tab);
				let b = c.read_u8().unwrap();
				visitor.visit_bool(b != 0)
			},
			Some(0x02) => {
				visitor.visit_string(self.parse_string())
			},
			Some(0x03) => {
                visitor.visit_map(MapVisitor::new(self, None))
			}
			Some(0x08) => {
				let mut tab = Vec::new();
				for _ in 0..4 {
					tab.push(self.reader.next().unwrap().unwrap());
				}
				let mut c = io::Cursor::new(tab);
				let nb = c.read_u32::<BigEndian>().unwrap();
                visitor.visit_map(MapVisitor::new(self, Some(nb)))
			}
			_ => visitor.visit_unit()
		}
		
	}
}

impl<'a, W> serde::Deserializer for &'a mut Deserializer<W>
	where W: Read,
{

	type Error = Error;

	fn deserialize<T: de::Visitor>(self, visitor : T) -> Result<T::Value, self::Error> {
		self.parse_value(visitor)
	}

	forward_to_deserialize! {
        bool u8 u16 u32 u64 i8 i16 i32 i64 f32 f64 char str string
        unit option seq seq_fixed_size bytes byte_buf map unit_struct
        newtype_struct tuple_struct struct struct_field tuple enum ignored_any
	}	
}

pub struct StringDeserializer<W> {
	pub reader: W,
}

impl<W> StringDeserializer<W>
	where W: Read
{
	fn parse_string(&mut self) -> String {
		let mut tab = Vec::new();
		for _ in 0..2 {
			tab.push(self.reader.next().unwrap().unwrap());
		}
		let mut c = io::Cursor::new(tab);
		let nb = c.read_u16::<BigEndian>().unwrap();
		let mut str = Vec::new();
		for _ in 0..nb {
			str.push(self.reader.next().unwrap().unwrap());
		}
		let s = String::from_utf8(str).unwrap();
		s
	}
}

impl<'a, W> serde::Deserializer for &'a mut StringDeserializer<W>
	where W: Read,
{

	type Error = Error;

	fn deserialize<T: de::Visitor>(self, visitor : T) -> Result<T::Value, self::Error> {
		visitor.visit_string(self.parse_string())
	}

	forward_to_deserialize! {
        bool u8 u16 u32 u64 i8 i16 i32 i64 f32 f64 char str string
        unit option seq seq_fixed_size bytes byte_buf map unit_struct
        newtype_struct tuple_struct struct struct_field tuple enum ignored_any
	}	
}

struct MapVisitor<'a, R: Read + 'a> {
    de: &'a mut Deserializer<R>,
    size: u32,
    map: bool
}

impl<'a, R: Read + 'a> MapVisitor<'a, R> {
    fn new(de: &'a mut Deserializer<R>, size: Option<u32>) -> Self {
    	match size {
    		None => {
        		MapVisitor {
            		de: de,
            		size: 0,
            		map: true
        		}    			
    		},
    		Some(size) => {
    			MapVisitor {
    				de: de,
    				size: size,
    				map: false
    			}
    		}
    	}
    }
}

impl<'a, R: Read + 'a> de::MapVisitor for MapVisitor<'a, R> {
    type Error = Error;

    fn visit_key_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, self::Error> 
        where T: de::DeserializeSeed,
   {
   		let mut de = StringDeserializer{reader: self.de.reader.copy()};
   		let ret = seed.deserialize(&mut de).unwrap();
   		match self.de.parse_string_or_end() {
	   		None => if self.map || self.size == 0 { Ok(None) } else { Ok(None) /* Error */ },
	   		_ => if self.map || self.size != 0 { if !self.map {self.size -= 1;} Ok(Some(ret)) } else { Ok(None) /* Error */ }
   		}
   }

    fn visit_value_seed<T>(&mut self, seed: T) -> Result<T::Value, self::Error> 
        where T: de::DeserializeSeed,
   {
   		seed.deserialize(&mut *self.de)
   }

   fn size_hint(&self) -> (usize, Option<usize>) {
	   	if self.map {
   			(0, None)
	   	} else {
	   		(1, None)
	   	}
   }
}