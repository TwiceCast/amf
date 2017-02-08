extern crate serde;
extern crate byteorder;

use std::collections::BTreeMap;
use std::fmt;
use serde::de;
use std::convert::From;

pub type Map<K, V> = BTreeMap<K, V>;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Date {
    date: f64,
    time_zone: i16,
}

/// Value represente AMF type
#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Number(f64),
    Bool(bool),
    String(String),
    Object(Map<String, Value>),
    /// This type is not supported and is reserved for future use.
    Movieclip,
    Null,
    Undefined,
    Reference(u16),
    ECMAArray(Map<String, Value>),
    StrictArray(Vec<Value>),
    /// i16
    Date(Date),
    LongString(String),
    Unsupported,
    /// This type is not supported and is reserved for future use.
    Recordset,
    XMLDocument(String),
    /// Named objet
    TypedObject(String, Map<String, Value>),
}

impl Value {

    /// Returns true if the `Value` is a Number. Returns false otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// use amf::Value;
    ///
    /// assert!(Value::Number(2.56).is_number());
    ///
    /// assert!(!Value::Null.is_number());
    /// ```
    pub fn is_number(&self) -> bool {
        if let Value::Number(_) = *self {
            true
        } else {
            false
        }
    }

    /// If the `Value` is a Number, returns the associated f64.
    /// Returns None otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// use amf::Value;
    ///
    /// assert_eq!(Some(2.56), Value::Number(2.56).as_number());
    ///
    /// assert_eq!(None, Value::Null.as_number());
    /// ```
    pub fn as_number(&self) -> Option<f64> {
        if let Value::Number(val) = *self {
            Some(val)
        } else {
            None
        }
    }

    /// Returns true if the `Value` is a Numberean. Returns false otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// use amf::Value;
    ///
    /// assert!(Value::Bool(true).is_bool());
    ///
    /// assert!(!Value::Null.is_bool());
    /// ```
    pub fn is_bool(&self) -> bool {
        if let Value::Bool(_) = *self {
            true
        } else {
            false
        }
    }

    /// If the `Value` is a Boolean, returns the associated bool.
    /// Returns None otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// use amf::Value;
    ///
    /// assert_eq!(Some(true), Value::Bool(true).as_bool());
    ///
    /// assert_eq!(None, Value::Null.as_bool());
    /// ```
    pub fn as_bool(&self) -> Option<bool> {
        if let Value::Bool(val) = *self {
            Some(val)
        } else {
            None
        }
    }

    pub fn is_null(&self) -> bool {
        if let Value::Null = *self {
            true
        }
        else {
            false
        }
    }
}

// static NULL: Value = Value::Null;

impl From<f64> for Value {
    fn from(v: f64) -> Value {
        Value::Number(v)
    }
}

impl From<bool> for Value {
    fn from(v: bool) -> Value {
        Value::Bool(v)
    }
}


impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Value::Number(v) => write!(f, "{}", v.to_string()),
            Value::Bool(v) => write!(f, "{}", v.to_string()),
            Value::String(ref s) => write!(f, "{}", s.to_string()),
            Value::Object(ref m) => {
                let _ = write!(f, "Object{}\n", "{"); //TODO make this clean
                for (k, v) in m {
                    let _ = write!(f, "{} => {},\n", k, v);
                }
                write!(f, "{}\n", "}") //TODO make this clean
            },
            Value::ECMAArray(ref m) => {
                let _ = write!(f, "Tab{}\n", "{"); //TODO make this clean
                for (k, v) in m {
                    let _ = write!(f, "{} => {},\n", k, v);
                }
                write!(f, "{}", "}") //TODO make this clean
            },
            Value::Null => write!(f, "Null"),
            _ => write!(f, "value")
        }
    }
}

impl serde::Serialize for Value {
    fn serialize<S>(&self, serializer: S) -> Result<<S as serde::Serializer>::Ok, S::Error> where S: serde::Serializer {
        match *self {
            Value::Number(v) => serializer.serialize_f64(v),
            Value::Bool(v) => serializer.serialize_bool(v),
            Value::String(ref v) => serializer.serialize_str(&v),
            Value::Object(ref m) => {
                let mut map_serializer = serializer.serialize_map(None).unwrap();
                use serde::ser::SerializeMap;
                for (k, v) in m {
                    let _ = map_serializer.serialize_key(k);
                    let _ = map_serializer.serialize_value(v);
                }
                map_serializer.end()
            },
            Value::Movieclip => serializer.serialize_unit(),
            Value::Null => serializer.serialize_unit(),
            Value::Undefined => serializer.serialize_unit(),
            Value::Reference(v) => serializer.serialize_u16(v),
            Value::ECMAArray(ref m) => {
                let mut map_serializer = serializer.serialize_map(Some(m.len())).unwrap();
                use serde::ser::SerializeMap;
                for (k, v) in m {
                    let _ = map_serializer.serialize_key(k);
                    let _ = map_serializer.serialize_value(v);
                }
                map_serializer.end()
            },
//            Value::StrictArray(ref m) => serializer.serialize_strict_array(&m),
            //Value::Date(v) => serializer.serialize_u16(v),
            Value::Unsupported => serializer.serialize_unit(),
            Value::Recordset => serializer.serialize_unit(),
            Value::XMLDocument(ref v) => serializer.serialize_str(&v),
            //TypedObject(String, Map<String, Value>),*/
            _ => serializer.serialize_unit(),
        }
    }
}


impl de::Deserialize for Value {
    fn deserialize<D>(deserializer: D) -> Result<Value, D::Error>    
        where D: de::Deserializer
    {
        struct ValueVisitor;

        impl de::Visitor for ValueVisitor {
            type Value = Value;

            fn expecting(&self, fmt: &mut fmt::Formatter) ->Result<(), fmt::Error>
            {
                fmt.write_str("AMF string")
            }

            fn visit_bool<E>(self, value: bool) -> Result<Value, E>
                where E: de::Error,
            {
                Ok(Value::Bool(value))
            }

            fn visit_f64<E>(self, value: f64) -> Result<Value, E>
                where E: de::Error,
            {
                Ok(Value::Number(value))
            }

            fn visit_unit<E>(self) -> Result<Value, E>
                where E: de::Error,
            {
                Ok(Value::Null)
            }

            fn visit_string<E>(self, value: String) -> Result<Value, E>
                where E: de::Error,
            {
                Ok(Value::String(value))
            }

            fn visit_map<V>(self, mut visitor: V) -> Result<Value, V::Error>
                where V : de::MapVisitor
            {
                let mut values = Map::new();

                while let Some((key, value)) = try!(visitor.visit()) {
                    values.insert(key, value);
                }

                if visitor.size_hint().0 == 0 {
                    Ok(Value::Object(values))
                } else {
                    Ok(Value::ECMAArray(values))
                }
            }
        }

        deserializer.deserialize(ValueVisitor)
    }
}
