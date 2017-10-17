use std::collections::HashMap;

/// Representation of an NBT value.
#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    /// Represents a TAG_Byte tag.
    Byte(i8),
    /// Represents a TAG_Short tag.
    Short(i16),
    /// Represents a TAG_Int tag.
    Int(i32),
    /// Represents a TAG_Long tag.
    Long(i64),
    /// Represents a TAG_Float tag.
    Float(f32),
    /// Represents a TAG_Double tag.
    Double(f64),
    /// Represents a TAG_Byte_Array tag.
    ByteArray(Vec<i8>),
    /// Represents a TAG_String tag.
    String(String),
    /// Represents a TAG_List tag.
    List(Vec<Value>),
    /// Represents a TAG_Compound tag.
    Compound(HashMap<String, Value>),
    /// Represents a TAG_Int_Array tag.
    IntArray(Vec<i32>),
    /// Represents a TAG_Long_Array tag.
    LongArray(Vec<i64>),
}

impl Value {
    pub fn as_byte(&self) -> Option<i8> {
        match *self {
            Value::Byte(b) => Some(b),
            _ => None,
        }
    }

    pub fn is_byte(&self) -> bool {
        self.as_byte().is_some()
    }

    pub fn as_short(&self) -> Option<i16> {
        match *self {
            Value::Short(s) => Some(s),
            _ => None,
        }
    }

    pub fn is_short(&self) -> bool {
        self.as_short().is_some()
    }

    pub fn as_int(&self) -> Option<i32> {
        match *self {
            Value::Int(i) => Some(i),
            _ => None,
        }
    }

    pub fn is_int(&self) -> bool {
        self.as_int().is_some()
    }

    pub fn as_long(&self) -> Option<i64> {
        match *self {
            Value::Long(l) => Some(l),
            _ => None,
        }
    }

    pub fn is_long(&self) -> bool {
        self.as_long().is_some()
    }

    pub fn as_float(&self) -> Option<f32> {
        match *self {
            Value::Float(f) => Some(f),
            _ => None,
        }
    }

    pub fn is_float(&self) -> bool {
        self.as_float().is_some()
    }

    pub fn as_double(&self) -> Option<f64> {
        match *self {
            Value::Double(d) => Some(d),
            _ => None,
        }
    }

    pub fn is_double(&self) -> bool {
        self.as_double().is_some()
    }

    pub fn as_byte_array(&self) -> Option<&Vec<i8>> {
        match *self {
            Value::ByteArray(ref ba) => Some(ba),
            _ => None,
        }
    }

    pub fn as_byte_array_mut(&mut self) -> Option<&mut Vec<i8>> {
        match *self {
            Value::ByteArray(ref mut ba) => Some(ba),
            _ => None,
        }
    }

    pub fn is_byte_array(&self) -> bool {
        self.as_byte_array().is_some()
    }

    pub fn as_string(&self) -> Option<&String> {
        match *self {
            Value::String(ref s) => Some(s),
            _ => None,
        }
    }

    pub fn as_string_mut(&mut self) -> Option<&mut String> {
        match *self {
            Value::String(ref mut s) => Some(s),
            _ => None,
        }
    }

    pub fn is_string(&self) -> bool {
        self.as_string().is_some()
    }

    pub fn as_list(&self) -> Option<&Vec<Value>> {
        match *self {
            Value::List(ref l) => Some(l),
            _ => None,
        }
    }

    pub fn as_list_mut(&mut self) -> Option<&mut Vec<Value>> {
        match *self {
            Value::List(ref mut l) => Some(l),
            _ => None,
        }
    }

    pub fn is_list(&self) -> bool {
        self.as_list().is_some()
    }

    pub fn as_compound(&self) -> Option<&HashMap<String, Value>> {
        match *self {
            Value::Compound(ref c) => Some(c),
            _ => None,
        }
    }

    pub fn as_compound_mut(&mut self) -> Option<&mut HashMap<String, Value>> {
        match *self {
            Value::Compound(ref mut c) => Some(c),
            _ => None,
        }
    }

    pub fn is_compound(&self) -> bool {
        self.as_compound().is_some()
    }

    pub fn as_int_array(&self) -> Option<&Vec<i32>> {
        match *self {
            Value::IntArray(ref ia) => Some(ia),
            _ => None,
        }
    }

    pub fn as_int_array_mut(&mut self) -> Option<&mut Vec<i32>> {
        match *self {
            Value::IntArray(ref mut ia) => Some(ia),
            _ => None,
        }
    }

    pub fn is_int_array(&self) -> bool {
        self.as_int_array().is_some()
    }

    pub fn as_long_array(&self) -> Option<&Vec<i64>> {
        match *self {
            Value::LongArray(ref la) => Some(la),
            _ => None,
        }
    }

    pub fn as_long_array_mut(&mut self) -> Option<&mut Vec<i64>> {
        match *self {
            Value::LongArray(ref mut la) => Some(la),
            _ => None,
        }
    }

    pub fn is_long_array(&self) -> bool {
        self.as_long_array().is_some()
    }
}

macro_rules! impl_from {
    ($variant:ident : $T:ty) => {
        impl From<$T> for Value {
            #[inline]
            fn from(val: $T) -> Value {
                Value::$variant(val.into())
            }
        }
    }
}

impl<'a> From<&'a str> for Value {
    #[inline]
    fn from(val: &'a str) -> Value {
        Value::String(val.to_string())
    }
}

impl_from!(Byte: i8);
impl_from!(Short: i16);
impl_from!(Int: i32);
impl_from!(Long: i64);
impl_from!(Float: f32);
impl_from!(Double: f64);
impl_from!(ByteArray: Vec<i8>);
impl_from!(String: String);
impl_from!(List: Vec<Value>);
impl_from!(Compound: HashMap<String, Value>);
impl_from!(IntArray: Vec<i32>);
impl_from!(LongArray: Vec<i64>);