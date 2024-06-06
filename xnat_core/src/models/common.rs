//! Common utilities and implementation shared
//! among models within this sub-crate. Allows for
//! specific behavior when processing data to and
//! from JSON to some Model.
use std::marker::PhantomData;

use serde::{de::Visitor, Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct ResultData<T> {
    #[serde(flatten)]
    results: Vec<T>
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ResultSet<T> {
    #[serde(rename = "Result")]
    result: ResultData<T>
}

#[derive(Default)]
pub struct ModelPropertyVisitor<T>(PhantomData<T>);

impl<'de> Visitor<'de> for ModelPropertyVisitor<bool> {
    type Value = bool;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid boolean value")
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::MapAccess<'de>,
    {
        Ok(map.next_entry::<String, Self::Value>()?.unwrap().1)
    }

    fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(v)
    }
}

impl<'de> Visitor<'de> for ModelPropertyVisitor<String> {
    type Value = String;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid string value")
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::MapAccess<'de>,
    {
        Ok(map.next_entry::<String, Self::Value>()?.unwrap().1)
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(String::from(v))
    }
}

impl<'de> Visitor<'de> for ModelPropertyVisitor<u64> {
    type Value = u64;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid numerical value")
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::MapAccess<'de>,
    {
        Ok(map.next_entry::<String, Self::Value>()?.unwrap().1)
    }

    fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(v)
    }
}

impl<'de, T: Deserialize<'de>> Visitor<'de> for ModelPropertyVisitor<Vec<T>> {
    type Value = Vec<T>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid vector or array")
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::SeqAccess<'de>,
    {
        let mut ret = vec![];
        while let Some(v) = seq.next_element()? {
            ret.push(v);
        }
        Ok(ret)
    }
}
