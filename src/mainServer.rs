extern crate byteorder;
mod lib;

use std::io::{Read, BufReader, Write, Cursor};
use std::net::{TcpListener, TcpStream};
use std::str;
use std::fmt;
use byteorder::{BigEndian, ReadBytesExt};

#[derive(Debug)]
struct Type<'a> {
	type_object: lib::Value<'a>,
}

impl<'a> fmt::Display for Type<'a> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}", self.type_object)
	}
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:5432").unwrap();
    
    let mut stream = listener.accept().unwrap().0;
    stream.write_all("toto".as_bytes()).unwrap();
    handle_request(stream);
}

fn read_ecma_array(array: &[u8]) -> (usize, Type) {
	let mut arr: Vec<(&str, lib::Value)> = Vec::new();
	let count = read_size_32(&array[..4]);
	let mut size = 4;
	for _ in 0..count {
		let size_propriety = read_size(&array[size..size + 2]);
		let propiety = read_propriety(&array[size + 2..], size_propriety);
		size += propiety.0;
		arr.push(propiety.1);
	}
	if read_size(&array[size..size + 2]) != 0 && array[size + 2] != 0x09 {
		panic!("Bad end of ecma array");
	}
	let t: Type = Type {type_object: lib::Value::Object(arr) };
	return (size + 3, t);
}

fn read_type(array: &[u8]) -> (usize, Type) {
	let value;
	let t: Type;

	match array[0] {
    	0x00 => {
    		let nbr = read_number(&array[1..]);
    		t = nbr.1;
	    	value = (1 + nbr.0, t);
    	},
    	0x01 => {
    		let b = read_bool(&array[1..]);
    		t = b.1;
	    	value = (1 + b.0, t);
    	},
    	0x02 => {
    		let string = read_string(&array[1..]);
    		t = string.1;
	    	value = (1 + string.0, t);
    	},
    	0x03 =>{
    		let obj = read_object(&array[1..]);
    		t = obj.1;
    		value = (1 + obj.0, t);
    	},
    	0x08 => {
			let arr = read_ecma_array(&array[1..]);
    		t = arr.1;
			value = (1 + arr.0, t);
    	},
    	nb => {
    		panic!("Unknown op: {:?}", nb);
    	},
    }
    return value;
}

fn read_bool(array: &[u8]) -> (usize, Type) {
	match array[0] {
		0x00 => (1, Type {type_object: lib::Value::Bool(false)}),
		0x01 => (1, Type {type_object: lib::Value::Bool(true)}),
		_ => panic!("Unknown boolean"),
	}
}

fn read_string(array: &[u8]) -> (usize, Type) {
	let size = read_size(array);
	let t: Type = Type {type_object : lib::Value::String(str::from_utf8(&array[2..size + 2]).unwrap())};
	return (size + 2, t);
}

fn read_number(array: &[u8]) -> (usize, Type) {
	let mut cursor = Cursor::new(&array[0..8]);
	let nb = cursor.read_f64::<BigEndian>().unwrap();
	let t: Type = Type { type_object : lib::Value::Number(nb)};
	return (8, t);
}

fn read_size(array: &[u8]) -> usize {
	return array[0] as usize * 256 + array[1] as usize
}

fn read_size_32(array: &[u8]) -> usize {
	let mut size = 0;
	for i in 0..4 {
		size = size * 256 + array[i] as usize;
	}
	return size
}

fn read_propriety(array: &[u8], size: usize) -> (usize, (&str, lib::Value)) {
	let key = str::from_utf8(&array[..size]).unwrap();
	let value = read_type(&array[size..]);
	return (2 + size + value.0, (key, value.1.type_object));
}

fn read_object(array: &[u8]) -> (usize, Type) {
	let mut i = 0;
	let t: Type;
	let mut obj: Vec<(&str, lib::Value)> = Vec::new();
	while i < array.len() {
		let size = read_size(&array[i..i + 2]);
		match size {
			0 => {
				t = Type { type_object : lib::Value::Object(obj) };
				return (i + 3, t);
			},
			_ => {
				let propriety = read_propriety(&array[i + 2..], size);
				i += propriety.0;
				obj.push(propriety.1);
			},
		}
	}
	panic!("Object whithout end");
}

fn handle_request(stream: TcpStream) {
    let mut reader = BufReader::new(stream);
    let mut array = [0; 526];
    let size = reader.read(&mut array).unwrap();
    let mut i = 0;
    while i < size {
    	let propriety = read_type(&array[i..]);
    	i += propriety.0;
    	println!("{}", propriety.1);
    }
    println!("(end)");
}