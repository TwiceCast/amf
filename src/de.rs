extern crate byteorder;

use std::io;
use serde::de;
use serde;
use reader::Read;
use error::Error;
use self::byteorder::{BigEndian, ReadBytesExt};

pub struct Deserializer<W> {
	pub reader: W,
}

impl<W> Deserializer<W>
	where W: Read
{
	fn next_value_or_eof(&mut self) -> Result<u8, self::Error>
	{
		match try!(self.reader.next()) {
			None => Err(Error::UnexpectedEOF),
			Some(c) => Ok(c),
		}
	}

	fn parse_string(&mut self) -> Result<String, self::Error> {
		let mut tab = Vec::new();
		for _ in 0..2 {
			let c = try!(self.next_value_or_eof());
			tab.push(c);
		}
		let mut c = io::Cursor::new(tab);
		let nb = c.read_u16::<BigEndian>().unwrap();
		let mut str = Vec::new();
		for _ in 0..nb {
			match try!(self.reader.next()) {
				None => return Err(Error::InvalidSize),
				Some(c) => str.push(c),
			}
		}
		Ok(String::from_utf8(str).unwrap())
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
					let c = try!(self.next_value_or_eof());
					tab.push(c);
				}
				let mut c = io::Cursor::new(tab);
				let nb = c.read_f64::<BigEndian>().unwrap();
				visitor.visit_f64(nb)
			},
			Some(0x01) => {
				let c = try!(self.next_value_or_eof());
				let tab = vec![c];
				let mut cursor = io::Cursor::new(tab);
				let b = cursor.read_u8().unwrap();
				visitor.visit_bool(b != 0)						
			},
			Some(0x02) => {
				let s = try!(self.parse_string());
				visitor.visit_string(s)
			},
			Some(0x03) => {
                visitor.visit_map(MapVisitor::new(self, None))
			}
			Some(0x08) => {
				let mut tab = Vec::new();
				for _ in 0..4 {
					let c = try!(self.next_value_or_eof());
					tab.push(c);
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
	fn next_value_or_eof(&mut self) -> Result<u8, self::Error>
	{
		match try!(self.reader.next()) {
			None => Err(Error::UnexpectedEOF),
			Some(c) => Ok(c),
		}
	}

	fn parse_string(&mut self) -> Result<String, self::Error> {
		let mut tab = Vec::new();
		for _ in 0..2 {
			let c = try!(self.next_value_or_eof());
			tab.push(c);
		}
		let mut c = io::Cursor::new(tab);
		let nb = c.read_u16::<BigEndian>().unwrap();
		let mut str = Vec::new();
		for _ in 0..nb {
			match try!(self.reader.next()) {
				None => return Err(Error::InvalidSize),
				Some(c) => str.push(c),
			}
		}
		Ok(String::from_utf8(str).unwrap())
	}
}

impl<'a, W> serde::Deserializer for &'a mut StringDeserializer<W>
	where W: Read,
{

	type Error = Error;

	fn deserialize<T: de::Visitor>(self, visitor : T) -> Result<T::Value, self::Error> {
		let s = try!(self.parse_string());
		visitor.visit_string(s)
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
   		let ret = seed.deserialize(&mut de);
   		match self.de.parse_string_or_end() {
	   		None => if self.map || self.size == 0 { Ok(None) } else { Err(Error::InvalidSize) },
	   		_ => if self.map || self.size != 0 { if !self.map {self.size -= 1;} Ok(Some(ret.unwrap())) } else { Err(Error::InvalidSize) }
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