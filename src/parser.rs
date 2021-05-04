use serde::{
    de::{DeserializeOwned, Error as DeError, IntoDeserializer},
    Deserialize, Deserializer,
};
use std::{fmt::Display, str::FromStr};

pub fn deserialize_str_opt<'de, D, T>(deserializer: D) -> Result<Option<T>, D::Error>
where
    D: Deserializer<'de>,
    T: DeserializeOwned,
{
    let s = String::deserialize(deserializer)?;
    if s.is_empty() {
        Ok(None)
    } else {
        let val = T::deserialize(s.into_deserializer())?;
        Ok(Some(val))
    }
}

pub fn from_str_opt<'de, D, T>(deserializer: D) -> Result<Option<T>, D::Error>
where
    D: Deserializer<'de>,
    T: FromStr,
    T::Err: Display,
{
    let s = String::deserialize(deserializer)?;
    if s.is_empty() {
        Ok(None)
    } else {
        Ok(Some(s.parse().map_err(|e| D::Error::custom(e))?))
    }
}

pub fn from_str<'de, D, T>(deserializer: D) -> Result<T, D::Error>
where
    D: Deserializer<'de>,
    T: FromStr,
    T::Err: Display,
{
    let s = String::deserialize(deserializer)?;
    s.parse().map_err(|e| D::Error::custom(e))
}
