use serde_json::{Number, Value};

use crate::error::ConversionError;

#[doc(hidden)]
pub trait FromValue: Sized {
    fn try_from(v: &Value) -> Result<Self, ConversionError>;
}

impl FromValue for String {
    fn try_from(v: &Value) -> Result<Self, ConversionError> {
        match v {
            Value::String(s) => Ok(s.clone()),
            _ => Err(ConversionError {}),
        }
    }
}

impl<T> FromValue for Vec<T>
where
    T: FromValue,
{
    fn try_from(v: &Value) -> Result<Self, ConversionError> {
        match v {
            Value::Array(arr) => arr
                .iter()
                .map(T::try_from)
                .collect::<Result<Vec<_>, ConversionError>>(),
            _ => T::try_from(v).map(|value| vec![value]),
        }
    }
}

#[doc(hidden)]
pub trait ToValue: Sized {
    fn into_value(self) -> Value;
}

impl ToValue for &str {
    fn into_value(self) -> Value {
        Value::String(self.to_string())
    }
}

impl ToValue for &String {
    fn into_value(self) -> Value {
        Value::String(self.to_string())
    }
}

impl ToValue for String {
    fn into_value(self) -> Value {
        Value::String(self)
    }
}

impl ToValue for usize {
    fn into_value(self) -> Value {
        Value::Number(Number::from(self))
    }
}

impl ToValue for isize {
    fn into_value(self) -> Value {
        Value::Number(Number::from(self))
    }
}

impl ToValue for u32 {
    fn into_value(self) -> Value {
        Value::Number(Number::from(self))
    }
}

impl ToValue for u64 {
    fn into_value(self) -> Value {
        Value::Number(Number::from(self))
    }
}


impl<T> ToValue for Vec<T>
where
    T: ToValue,
{
    fn into_value(self) -> Value {
        let values = self.into_iter().map(T::into_value).collect();

        Value::Array(values)
    }
}
