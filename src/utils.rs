use serde::de::Error;
use serde::{Deserialize, Deserializer};
use serde_json::Value;
use std::fmt::Formatter;
use std::str::FromStr;

pub fn string_or_number_to_u8<'de, D>(deserializer: D) -> Result<u8, D::Error>
where
    D: serde::Deserializer<'de>,
{
    struct StringOrNumberVisitor;

    impl<'de> serde::de::Visitor<'de> for StringOrNumberVisitor {
        type Value = u8;

        fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
            formatter.write_str("a string or an integer")
        }

        fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: Error,
        {
            u8::from_str(v).map_err(serde::de::Error::custom)
        }

        fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
        where
            E: Error,
        {
            if v <= u8::MAX as u64 {
                Ok(v as u8)
            } else {
                Err(E::custom(format!("u64 value {v} is out of range for u8")))
            }
        }
    }

    deserializer.deserialize_any(StringOrNumberVisitor)
}

pub fn string_or_number_to_u16<'de, D>(deserializer: D) -> Result<u16, D::Error>
where
    D: serde::Deserializer<'de>,
{
    struct StringOrNumberVisitor;

    impl<'de> serde::de::Visitor<'de> for StringOrNumberVisitor {
        type Value = u16;

        fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
            formatter.write_str("a string or an integer")
        }

        fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: Error,
        {
            u16::from_str(v).map_err(serde::de::Error::custom)
        }

        fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
        where
            E: Error,
        {
            if v <= u16::MAX as u64 {
                Ok(v as u16)
            } else {
                Err(E::custom(format!("u64 value {v} is out of range for u16")))
            }
        }
    }

    deserializer.deserialize_any(StringOrNumberVisitor)
}

pub fn option_string_or_number_to_u8<'de, D>(deserializer: D) -> Result<Option<u8>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    struct OptionStringOrNumberVisitor;

    impl<'de> serde::de::Visitor<'de> for OptionStringOrNumberVisitor {
        type Value = Option<u8>;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("an optional u8, either as a number, a string, or null")
        }

        fn visit_none<E>(self) -> Result<Self::Value, E>
        where
            E: Error,
        {
            Ok(None)
        }

        fn visit_unit<E>(self) -> Result<Self::Value, E>
        where
            E: Error,
        {
            Ok(None)
        }

        fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            // Delegate to the exisiting deserializer
            super::string_or_number_to_u8(deserializer).map(Some)
        }
    }

    deserializer.deserialize_option(OptionStringOrNumberVisitor)
}

pub fn option_string_or_number_to_u16<'de, D>(deserializer: D) -> Result<Option<u16>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    struct OptionStringOrNumberVisitor;

    impl<'de> serde::de::Visitor<'de> for OptionStringOrNumberVisitor {
        type Value = Option<u16>;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("an optional u8, either as a number, a string, or null")
        }

        fn visit_none<E>(self) -> Result<Self::Value, E>
        where
            E: Error,
        {
            Ok(None)
        }

        fn visit_unit<E>(self) -> Result<Self::Value, E>
        where
            E: Error,
        {
            Ok(None)
        }

        fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            // Delegate to the exisiting deserializer
            super::string_or_number_to_u16(deserializer).map(Some)
        }
    }

    deserializer.deserialize_option(OptionStringOrNumberVisitor)
}

pub fn bool_from_int_or_bool<'de, D>(deserializer: D) -> Result<Option<bool>, D::Error>
where
    D: Deserializer<'de>,
{
    let v: Option<Value> = Option::deserialize(deserializer)?;
    match v {
        Some(Value::Bool(b)) => Ok(Some(b)),
        Some(Value::Number(n)) => {
            if let Some(i) = n.as_i64() {
                Ok(Some(i != 0))
            } else {
                Err(serde::de::Error::custom("Invalid number for bool"))
            }
        }
        Some(Value::Null) | None => Ok(None),
        _ => Err(serde::de::Error::custom("Expected bool or int")),
    }
}
