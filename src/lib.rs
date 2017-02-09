#[macro_use]
extern crate serde;

pub use self::ser::Serializer;
pub use self::de::Deserializer;
pub use self::reader::SliceReader;
pub use self::value::Value;
pub use self::error::Error;

pub mod ser;
pub mod de;
pub mod value;
pub mod reader;
pub mod error;

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
    fn serialize_true_bool() {
    	let v = Value::Bool(true);
		let mut ser = Vec::with_capacity(128);
		{
			let serializer = self::Serializer {writer: &mut ser };	
 		   	use serde::Serialize;
			let _ = v.serialize(serializer);	
		}
		let true_bool = vec![0x01, 0x01];
    	assert_eq!(ser, true_bool)
    }

	#[test]
    fn serialize_false_bool() {
    	let v = Value::Bool(false);
		let mut ser = Vec::with_capacity(128);
		{
			let serializer = self::Serializer {writer: &mut ser };	
 		   	use serde::Serialize;
			let _ = v.serialize(serializer);	
		}
		let false_bool = vec![0x01, 0x00];
    	assert_eq!(ser, false_bool)
    }

	#[test]
    fn deserialize_true_bool() {
    	let v = [0x01, 0x01];
	    let s = SliceReader::new(&v);
	    let mut de = Deserializer{reader: s};
	    use serde::Deserialize;
	    let v = Value::deserialize(&mut de).unwrap();
	    assert_eq!(v, Value::Bool(true))
    }

	#[test]
    fn deserialize_false_bool() {
    	let v = [0x01, 0x00];
	    let s = SliceReader::new(&v);
	    let mut de = Deserializer{reader: s};
	    use serde::Deserialize;
	    let v = Value::deserialize(&mut de).unwrap();
	    assert_eq!(v, Value::Bool(false))
    }

	#[test]
	#[should_panic]
    fn deserialize_shorter_bool() {
    	let v = [0x01];
	    let s = SliceReader::new(&v);
	    let mut de = Deserializer{reader: s};
	    use serde::Deserialize;
	    let _ = Value::deserialize(&mut de).unwrap();
    }

	#[test]
    fn serialize_zero() {
    	let v = Value::Number(0.);
		let mut ser = Vec::with_capacity(128);
		{
			let serializer = self::Serializer {writer: &mut ser };	
 		   	use serde::Serialize;
			let _ = v.serialize(serializer);	
		}
		let zero = vec![0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
    	assert_eq!(ser, zero)
    }

	#[test]
    fn serialize_42() {
    	let v = Value::Number(42.);
		let mut ser = Vec::with_capacity(128);
		{
			let serializer = self::Serializer {writer: &mut ser };	
 		   	use serde::Serialize;
			let _ = v.serialize(serializer);	
		}
		let zero = vec![0x00, 0x40, 0x45, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
    	assert_eq!(ser, zero)
    }

	#[test]
    fn deserialize_zero() {
    	let v = [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
	    let s = SliceReader::new(&v);
	    let mut de = Deserializer{reader: s};
	    use serde::Deserialize;
	    let v = Value::deserialize(&mut de).unwrap();
	    assert_eq!(v, Value::Number(0.))
    }

	#[test]
    fn deserialize_42() {
    	let v = [0x00, 0x40, 0x45, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
	    let s = SliceReader::new(&v);
	    let mut de = Deserializer{reader: s};
	    use serde::Deserialize;
	    let v = Value::deserialize(&mut de).unwrap();
	    assert_eq!(v, Value::Number(42.))
    }

	#[test]
	#[should_panic]
    fn deserialize_shorter_number() {
    	let v = [0x00, 0x00, 0x00];
	    let s = SliceReader::new(&v);
	    let mut de = Deserializer{reader: s};
	    use serde::Deserialize;
	    let _ = Value::deserialize(&mut de).unwrap();
    }

	#[test]
    fn serialize_string_empty() {
    	let v = Value::String("".to_string());
		let mut ser = Vec::with_capacity(128);
		{
			let serializer = self::Serializer {writer: &mut ser };	
 		   	use serde::Serialize;
			let _ = v.serialize(serializer);	
		}
		let empty = vec![0x02, 0x00, 0x00];
    	assert_eq!(ser, empty)
    }

	#[test]
    fn serialize_string_hello() {
    	let v = Value::String("hello".to_string());
		let mut ser = Vec::with_capacity(128);
		{
			let serializer = self::Serializer {writer: &mut ser };	
 		   	use serde::Serialize;
			let _ = v.serialize(serializer);	
		}
		let empty = vec![0x02, 0x00, 0x05, 0x68, 0x65, 0x6C, 0x6C, 0x6F];
    	assert_eq!(ser, empty)
    }

	#[test]
    fn deserialize_string_empty() {
    	let v = [0x02, 0x00, 0x00];
	    let s = SliceReader::new(&v);
	    let mut de = Deserializer{reader: s};
	    use serde::Deserialize;
	    let v = Value::deserialize(&mut de).unwrap();
	    assert_eq!(v, Value::String("".to_string()))

    }

	#[test]
    fn deserialize_string_hello() {
    	let v = [0x02, 0x00, 0x05, 0x68, 0x65, 0x6C, 0x6C, 0x6F];
	    let s = SliceReader::new(&v);
	    let mut de = Deserializer{reader: s};
	    use serde::Deserialize;
	    let v = Value::deserialize(&mut de).unwrap();
	    assert_eq!(v, Value::String("hello".to_string()))

    }

	#[test]
	#[should_panic]
    fn deserialize_shorter_string() {
    	let v = [0x02, 0x00];
	    let s = SliceReader::new(&v);
	    let mut de = Deserializer{reader: s};
	    use serde::Deserialize;
	    let _ = Value::deserialize(&mut de).unwrap();
    }

	#[test]
	#[should_panic]
    fn deserialize_string_size_too_long() {
    	let v = [0x02, 0x00, 0x03, 0x00];
	    let s = SliceReader::new(&v);
	    let mut de = Deserializer{reader: s};
	    use serde::Deserialize;
	    let _ = Value::deserialize(&mut de).unwrap();
    }

	#[test]
    fn serialize_object_empty() {
    	let map = value::Map::new();
    	let v = Value::Object(map);
		let mut ser = Vec::with_capacity(128);
		{
			let serializer = self::Serializer {writer: &mut ser };	
 		   	use serde::Serialize;
			let _ = v.serialize(serializer);	
		}
		let object = vec![0x03, 0x00, 0x00, 0x09];
    	assert_eq!(ser, object)
    }

	#[test]
    fn serialize_object_compound() {
    	let mut map = value::Map::new();
		map.insert("1".to_string(), Value::Number(42.));
		map.insert("2".to_string(), Value::String("hello".to_string()));
		map.insert("3".to_string(), Value::Bool(true));
    	let v = Value::Object(map);
		let mut ser = Vec::with_capacity(128);
		{
			let serializer = self::Serializer {writer: &mut ser };	
 		   	use serde::Serialize;
			let _ = v.serialize(serializer);	
		}
		let object = vec![0x03,
		 0x00, 0x01, 0x31, 0x00, 0x40, 0x45, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
		 0x00, 0x01, 0x32, 0x02, 0x00, 0x05, 0x68, 0x65, 0x6C, 0x6C, 0x6F,
		 0x00, 0x01, 0x33, 0x01, 0x01,
		 0x00, 0x00, 0x09];
    	assert_eq!(ser, object)
    }

	#[test]
    fn deserialize_object_empty() {
    	let v = [0x03, 0x00, 0x00, 0x09];
	    let s = SliceReader::new(&v);
	    let mut de = Deserializer{reader: s};
	    use serde::Deserialize;
	    let v = Value::deserialize(&mut de).unwrap();
    	let map = value::Map::new();
	    assert_eq!(v, Value::Object(map))
    }

	#[test]
    fn deserialize_object_compound() {
		let v = vec![0x03,
		 0x00, 0x01, 0x31, 0x00, 0x40, 0x45, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
		 0x00, 0x01, 0x32, 0x02, 0x00, 0x05, 0x68, 0x65, 0x6C, 0x6C, 0x6F,
		 0x00, 0x01, 0x33, 0x01, 0x01,
		 0x00, 0x00, 0x09];
	    let s = SliceReader::new(&v);
	    let mut de = Deserializer{reader: s};
	    use serde::Deserialize;
	    let v = Value::deserialize(&mut de).unwrap();
    	let mut map = value::Map::new();
		map.insert("1".to_string(), Value::Number(42.));
		map.insert("2".to_string(), Value::String("hello".to_string()));
		map.insert("3".to_string(), Value::Bool(true));
	    assert_eq!(v, Value::Object(map))
    }

	#[test]
	#[should_panic]
    fn deserialize_shorter_object() {
    	let v = [0x03];
	    let s = SliceReader::new(&v);
	    let mut de = Deserializer{reader: s};
	    use serde::Deserialize;
	    let _ = Value::deserialize(&mut de).unwrap();
    }

	#[test]
	fn serialize_array_empty() {
    	let map = value::Map::new();
    	let v = Value::ECMAArray(map);
		let mut ser = Vec::with_capacity(128);
		{
			let serializer = self::Serializer {writer: &mut ser };	
 		   	use serde::Serialize;
			let _ = v.serialize(serializer);	
		}
		let tab = vec![0x08, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x09];
    	assert_eq!(ser, tab)
	}

	#[test]
	fn serialize_array_with_value() {
    	let mut map = value::Map::new();
		map.insert("1".to_string(), Value::Number(42.));
		map.insert("2".to_string(), Value::Number(-42.));
    	let v = Value::ECMAArray(map);
		let mut ser = Vec::with_capacity(128);
		{
			let serializer = self::Serializer {writer: &mut ser };	
 		   	use serde::Serialize;
			let _ = v.serialize(serializer);	
		}
		let tab = vec![0x08, 0x00, 0x00, 0x00, 0x02,
		0x00, 0x01, 0x31, 0x00, 0x40, 0x45, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
		0x00, 0x01, 0x32, 0x00, 0xC0, 0x45, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
		0x00, 0x00, 0x09];
    	assert_eq!(ser, tab)
	}

	#[test]
	fn deserialize_array_empty() {
		let v = vec![0x08, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x09];
	    let s = SliceReader::new(&v);
	    let mut de = Deserializer{reader: s};
	    use serde::Deserialize;
	    let v = Value::deserialize(&mut de).unwrap();
    	let map = value::Map::new();
	    assert_eq!(v, Value::ECMAArray(map))
	}

	#[test]
	fn deserialize_array_with_value() {
		let v = vec![0x08, 0x00, 0x00, 0x00, 0x02,
		0x00, 0x01, 0x31, 0x00, 0x40, 0x45, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
		0x00, 0x01, 0x32, 0x00, 0xC0, 0x45, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
		0x00, 0x00, 0x09];
	    let s = SliceReader::new(&v);
	    let mut de = Deserializer{reader: s};
	    use serde::Deserialize;
	    let v = Value::deserialize(&mut de).unwrap();
    	let mut map = value::Map::new();
		map.insert("1".to_string(), Value::Number(42.));
		map.insert("2".to_string(), Value::Number(-42.));
	    assert_eq!(v, Value::ECMAArray(map))
	}

	#[test]
	#[should_panic]
    fn deserialize_shorter_array() {
    	let v = [0x08, 0x00];
	    let s = SliceReader::new(&v);
	    let mut de = Deserializer{reader: s};
	    use serde::Deserialize;
	    let _ = Value::deserialize(&mut de).unwrap();
    }

	#[test]
	#[should_panic]
    fn deserialize_array_size_too_long() {
    	let v = [0x08, 0x00, 0x00, 0x00, 0x03, 0x00, 0x00, 0x09];
	    let s = SliceReader::new(&v);
	    let mut de = Deserializer{reader: s};
	    use serde::Deserialize;
	    let _ = Value::deserialize(&mut de).unwrap();
    }

	#[test]
	#[should_panic]
    fn deserialize_array_size_too_short() {
    	let v = [0x08, 0x00, 0x00, 0x00, 0x00,
		0x00, 0x01, 0x31, 0x00, 0x40, 0x45, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
		0x00, 0x00, 0x09];
	    let s = SliceReader::new(&v);
	    let mut de = Deserializer{reader: s};
	    use serde::Deserialize;
	    let _ = Value::deserialize(&mut de).unwrap();
    }

	#[test]
	fn serialize_unit() {
    	let v = Value::Null;
		let mut ser = Vec::with_capacity(128);
		{
			let serializer = self::Serializer {writer: &mut ser };	
 		   	use serde::Serialize;
			let _ = v.serialize(serializer);	
		}
		let empty = vec![0x05];
    	assert_eq!(ser, empty)
	}

	#[test]
	fn deserialize_unit() {
		let v = vec![0x05];
	    let s = SliceReader::new(&v);
	    let mut de = Deserializer{reader: s};
	    use serde::Deserialize;
	    let v = Value::deserialize(&mut de).unwrap();
	    assert_eq!(v, Value::Null)		
	}

}
