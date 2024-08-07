use serde::de::Error;
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
                Err(E::custom(format!("u64 value {} is out of range for u8", v)))
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
                Err(E::custom(format!(
                    "u64 value {} is out of range for u16",
                    v
                )))
            }
        }
    }

    deserializer.deserialize_any(StringOrNumberVisitor)
}
