//! Common utilities and implementation shared
//! among models within this sub-crate. Allows for
//! specific behavior when processing data to and
//! from JSON to some Model.
use std::num::NonZeroU64;
use std::ops::Index;
use std::{collections::HashMap, slice::Iter};
use std::marker::PhantomData;

use serde::de::DeserializeOwned;
use serde::{de::Visitor, Deserialize, Serialize};

/// Custom type required to flexibly parse
/// non-zero `u64` integers either from a string
/// or an integer value.
#[derive(Clone, Copy, Debug, Serialize)]
#[serde(transparent)]
pub struct FlexU64(NonZeroU64);

struct FlexU64Visitor;

impl<'de> Visitor<'de> for FlexU64Visitor {
    type Value = FlexU64;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("integer or string")
    }

    fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
    {
        NonZeroU64::new(v)
            .ok_or(E::custom("invalid integer value"))
            .map(FlexU64)
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
    {
        match v.parse::<u64>() {
            Ok(val) => self.visit_u64(val),
            Err(_) => Err(E::custom("failed to parse integer"))
        }
    }
}

impl<'de> Deserialize<'de> for FlexU64 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>
    {
        deserializer.deserialize_any(FlexU64Visitor)
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum FormatSpecifier {
    Json,
    Xml,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Items<T> {
    pub items: Vec<Item<T>>
}

#[macro_export]
macro_rules! get_from_datafields {
    ($me:ident, $field_name:ident) => {
        &$me.data_fields.$field_name
    };
}

#[derive(Clone, Debug, Deserialize)]
pub struct Item<T> {
    #[serde(flatten)]
    pub children: Option<Vec<serde_json::Value>>,
    pub meta: Option<ItemMeta>,
    pub data_fields: T,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ItemMeta {
    pub create_event_id: Option<u64>,
    #[serde(rename = "isHistory")]
    pub is_history: Option<bool>,
    pub start_date: Option<String>,
    #[serde(rename = "xsi:type")]
    pub xsi_type: Option<String>,
}

impl<T> Item<T> {
    /// Try to get a child, as a particular model,
    /// from the contained children.
    pub fn child<C>(&self, index: usize) -> Option<C>
    where
        C: DeserializeOwned,
    {
        match &self.children {
            Some(children) => children
                .get(index)
                .and_then(|c| serde_json::from_value(c.clone()).expect("model deserialized")),
            None => None
        }
    }

    pub fn child_count(&self) -> usize {
        match &self.children {
            Some(c) => c.len(),
            None    => 0
        }
    }

    pub fn create_event_id(&self) -> &Option<u64> {
        match &self.meta {
            Some(m) => &m.create_event_id,
            None    => &None
        }
    }

    pub fn is_empty(&self) -> bool {
        self.child_count() > 0
    }

    pub fn is_history(&self) -> &Option<bool> {
        match &self.meta {
            Some(m) => &m.is_history,
            None    => &None
        }
    }

    pub fn start_date(&self) -> &Option<String> {
        match &self.meta {
            Some(m) => &m.start_date,
            None    => &None
        }
    }

    pub fn unwrap(&self) -> T
    where
        T: Clone
    {
        self.data_fields.clone()
    }

    pub fn xsi_type(&self) -> &Option<String> {
        match &self.meta {
            Some(m) => &m.xsi_type,
            None    => &None
        }
    }
}

impl<T> AsRef<T> for Item<T> {
    fn as_ref(&self) -> &T {
        &self.data_fields
    }
}

impl<T> Items<T> {
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    pub fn iter(&self) -> Iter<'_, Item<T>> {
        self.items.iter()
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }
}

impl<T> Index<usize> for Items<T> {
    type Output = Item<T>;

    fn index(&self, index: usize) -> &Self::Output {
        Index::index(&self.items, index)
    }
}

impl<T> IntoIterator for Items<T> {
    type IntoIter = std::vec::IntoIter<Item<T>>;
    type Item = Item<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.items.into_iter()
    }
}

#[derive(Debug, Deserialize)]
pub struct ResultSet<T> {
    #[serde(rename = "ResultSet")]
    pub data: ResultSetData<T>,
    #[serde(flatten)]
    pub metadata: ResultSetMeta,
}

#[derive(Debug, Deserialize)]
pub struct ResultSetData<T> {
    #[serde(rename = "Result")]
    pub result: Vec<T>,
}

#[derive(Debug, Deserialize)]
pub struct ResultSetMeta {
    #[serde(rename = "Columns")]
    pub columns: Option<HashMap<String, String>>,
    #[serde(rename = "title")]
    pub title: Option<String>,
    #[serde(rename = "totalRecords")]
    pub total_records: Option<u64>,
}

impl<T> ResultSet<T> {
    pub fn columns(&self) -> &Option<HashMap<String, String>> {
        &self.metadata.columns
    }

    pub fn is_empty(&self) -> bool {
        self.len() < 1
    }

    pub fn len(&self) -> usize {
        self
            .metadata
            .total_records
            .and_then(|tr| (tr as usize).into())
            .or(self.data.result.len().into())
            .unwrap()
    }

    pub fn results(&self) -> &Vec<T> {
        &self.data.result
    }

    pub fn title(&self) -> &Option<String> {
        &self.metadata.title
    }
}

/// Custom `serde::de::Visitor` for
/// `ModelProperty` types.
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
        formatter.write_str("a valid String value")
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

/// Trait for exposing the internal property value
/// of the implementing type.
pub trait ModelField<T> {
    /// Get the contained value of this property.
    fn property(&self) -> &T;
}
