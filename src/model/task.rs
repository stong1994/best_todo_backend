use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize, Serializer};

#[derive(Debug, Serialize, Deserialize)]
pub struct Task {
    #[serde(rename(serialize = "id", deserialize = "_id"), skip_serializing_if = "Option::is_none", serialize_with = "serialize_object_id")]
    pub id: Option<ObjectId>,
    pub title: String,
    pub is_done: bool,
    pub is_important: bool,
    pub is_urgent: bool,
}

pub fn serialize_object_id<S>(oid: &Option<ObjectId>, s: S) -> Result<S::Ok, S::Error> where S: Serializer {
    match oid.as_ref().map(|x| x.to_hex()) {
        Some(v) => s.serialize_str(&v),
        None => s.serialize_none()
    }
}