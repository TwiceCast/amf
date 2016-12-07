#[derive(Clone, PartialEq)]
pub enum Amf0 {
    Number(f64),
    Bool(bool),
    String(String),
    Object(BTreeMap<String, Value>),
    /// This type is not supported and is reserved for future use.
    Movieclip,
    Null,
    Undefined,
    Reference(u16),
    ECMArray(BTreeMap<String, Value>),
    ObjectEnd,
    StrictArray(Vec<Value>),
    Date(f64),
    LongString(String),
    Unsupported,
    Recordset,
    XMLDocumentjec(String),
    /// Named objet
    TypeObject(String, BTreeMap<String, Value>),
}
