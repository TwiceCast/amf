// In Rust we need to tell it where things are from,
// in this case we are using the read_to_string method
// so we need to bring in the std::io::Read
// module to the party. We also need TcpListener and
// TcpStream
extern crate amf;
extern crate serde;

use serde::Serialize;
use amf::{Serializer, Value};
use std::collections::BTreeMap;
use std::io::{Read, Write};
use std::net::{TcpStream};

fn main() {
	let mut tab = BTreeMap::new();
	let mut map = BTreeMap::new();
	tab.insert("tab1".to_string(), Value::Number(21.));
	tab.insert("tab2".to_string(), Value::Number(-42.5));
	map.insert("proprieties1".to_string(), Value::Number(42.));
	map.insert("proprieties2".to_string(), Value::String("Hello".to_string()));
	map.insert("proprieties3".to_string(), Value::Bool(true));
	map.insert("proprieties4".to_string(), Value::ECMAArray(tab));
	let val = Value::Object(map);
	println!("Envoi de {}", val.to_string());
	let mut ser = Vec::with_capacity(128);
	{
		let serializer = Serializer {writer: &mut ser };
		let _ = val.serialize(serializer);
	}
    let mut stream = TcpStream::connect("127.0.0.1:5432").unwrap();
    let mut response = [0; 128];
    let _ = stream.read(&mut response);
    let _ = stream.write(&ser);
}
