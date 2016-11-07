use std::collections::BTreeMap;

use std::convert::From;

pub type Map<K, V> = BTreeMap<K, V>;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Date {
    date: f64,
    time_zone: u16,
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
