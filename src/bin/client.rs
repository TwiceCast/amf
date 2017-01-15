// In Rust we need to tell it where things are from,
// in this case we are using the read_to_string method
// so we need to bring in the std::io::Read
// module to the party. We also need TcpListener and
// TcpStream
extern crate amf;
extern crate serde;

use serde::Serialize;
use amf::{Serializer, Value};
use std::io::{Read, Write};
use std::net::{TcpStream};

fn main() {
	let val = Value::Number(42.);
	println!("Envoi de {}", val.to_string());
	let ser = Vec::with_capacity(128);
	let mut serializer = Serializer {writer: ser };
	let _ = val.serialize(&mut serializer);
    let mut stream = TcpStream::connect("127.0.0.1:5432").unwrap();
    let mut response = [0; 128];
//    let mut message = [0x03, 0x00, 0x04, 0x6e, 0x61, 0x6d, 0x65, 0x02, 0x00, 0x04, 0x4d, 0x69, 0x6b, 0x65, 0x00, 0x03, 0x61, 0x67, 0x65, 0x00, 0x40, 0x3e, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x05, 0x61, 0x6c, 0x69, 0x61, 0x73, 0x02, 0x00, 0x04, 0x4d, 0x69, 0x6b, 0x65, 0x00, 0x00, 0x09];
 //   let mut message =[0x02, 0x00, 0x07, 0x5F, 0x72, 0x65, 0x73, 0x75, 0x6C, 0x74, 0x00, 0x3F, 0xF0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x03, 0x00, 0x06, 0x66, 0x6D, 0x73, 0x56, 0x65, 0x72, 0x02, 0x00, 0x0E, 0x46, 0x4D, 0x53, 0x2F, 0x33, 0x2C, 0x35, 0x2C, 0x35, 0x2C, 0x32, 0x30, 0x30, 0x34, 0x00, 0x0C, 0x63, 0x61, 0x70, 0x61, 0x62, 0x69, 0x6C, 0x69, 0x74, 0x69, 0x65, 0x73, 0x00, 0x40, 0x3F, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x04, 0x6D, 0x6F, 0x64, 0x65, 0x00, 0x3F, 0xF0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x09, 0x03, 0x00, 0x05, 0x6C, 0x65, 0x76, 0x65, 0x6C, 0x02, 0x00, 0x06, 0x73, 0x74, 0x61, 0x74, 0x75, 0x73, 0x00, 0x04, 0x63, 0x6F, 0x64, 0x65, 0x02, 0x00, 0x1D, 0x4E, 0x65, 0x74, 0x43, 0x6F, 0x6E, 0x6E, 0x65, 0x63, 0x74, 0x69, 0x6F, 0x6E, 0x2E, 0x43, 0x6F, 0x6E, 0x6E, 0x65, 0x63, 0x74, 0x2E, 0x53, 0x75, 0x63, 0x63, 0x65, 0x73, 0x73, 0x00, 0x0B, 0x64, 0x65, 0x73, 0x63, 0x72, 0x69, 0x70, 0x74, 0x69, 0x6F, 0x6E, 0x02, 0x00, 0x15, 0x43, 0x6F, 0x6E, 0x6E, 0x65, 0x63, 0x74, 0x69, 0x6F, 0x6E, 0x20, 0x73, 0x75, 0x63, 0x63, 0x65, 0x65, 0x64, 0x65, 0x64, 0x2E, 0x00, 0x04, 0x64, 0x61, 0x74, 0x61, 0x08, 0x00, 0x00, 0x00, 0x01, 0x00, 0x07, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6F, 0x6E, 0x02, 0x00, 0x0A, 0x33, 0x2C, 0x35, 0x2C, 0x35, 0x2C, 0x32, 0x30, 0x30, 0x34, 0x00, 0x00, 0x09, 0x00, 0x08, 0x63, 0x6C, 0x69, 0x65, 0x6E, 0x74, 0x69, 0x64, 0x00, 0x41, 0xD7, 0x9B, 0x78, 0x7C, 0xC0, 0x00, 0x00, 0x00, 0x0E, 0x6F, 0x62, 0x6A, 0x65, 0x63, 0x74, 0x45, 0x6E, 0x63, 0x6F, 0x64, 0x69, 0x6E, 0x67, 0x00, 0x40, 0x08, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x04, 0x62, 0x6f, 0x6f, 0x6c, 0x01, 0x01, 0x00, 0x00, 0x09];
    let _ = stream.read(&mut response);
    let _ = stream.write(&serializer.writer);
//    println!("{:?}", str::from_utf8(&mut response));
}
