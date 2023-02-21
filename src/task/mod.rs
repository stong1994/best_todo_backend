mod handler;
pub mod service;

use bson::oid::ObjectId;
use core::fmt;
pub use handler::*;
use serde::{de, Deserialize, Deserializer, Serialize, Serializer};

pub fn serialize_object_id<S>(oid: &Option<ObjectId>, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match oid.as_ref().map(|x| x.to_hex()) {
        Some(v) => s.serialize_str(&v),
        None => s.serialize_none(),
    }
}

pub fn deserialize_object_id<'de, D>(deserializer: D) -> Result<Option<ObjectId>, D::Error>
where
    D: Deserializer<'de>,
{
    struct JsonOptionObjectIdVisitor;

    impl<'de> de::Visitor<'de> for JsonOptionObjectIdVisitor {
        type Value = Option<ObjectId>;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("an object id hash value")
        }

        fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            if v.is_empty() {
                return Ok(None);
            }
            Ok(ObjectId::with_string(v).ok())
        }
    }

    deserializer.deserialize_any(JsonOptionObjectIdVisitor)
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Task {
    #[serde(serialize_with = "serialize_object_id")]
    _id: Option<ObjectId>,
    title: String,
    is_done: bool,
    is_important: bool,
    is_urgent: bool,
}

impl Task {
    pub const TABLE_NAME: &'static str = "task";
}

#[derive(Deserialize, Serialize, Debug)]
pub struct TaskQuery {
    #[serde(deserialize_with = "deserialize_object_id", default)]
    _id: Option<ObjectId>,
    #[serde(default)]
    keyword: String,
}
